use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
};

use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use backend::{AppState, routes};
use cucumber::World as _;
use futures::FutureExt as _;
use serde::{Deserialize, de::DeserializeOwned};
use sqlx::{PgPool, postgres::PgPoolOptions};
use testcontainers::{
    ContainerAsync, GenericImage, ImageExt,
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
};
use tower::ServiceExt as _;

mod step_definitions;

const POSTGRES_DB: &str = "md_stack_cucumber";
const POSTGRES_USER: &str = "md_stack";
const POSTGRES_PASSWORD: &str = "md_stack";
const POSTGRES_PORT: u16 = 5432;

#[derive(Default, cucumber::World)]
pub(crate) struct ApiWorld {
    database: Option<TestDatabase>,
    app: Option<Router>,
    pub(crate) last_response: Option<TestResponse>,
    pub(crate) request_count: usize,
    pub(crate) home: HomeFilterContext,
}

struct TestDatabase {
    container: ContainerAsync<GenericImage>,
    pool: PgPool,
    url: String,
}

#[derive(Debug)]
pub(crate) struct TestResponse {
    pub(crate) status: StatusCode,
    pub(crate) body: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub(crate) struct EntityOption {
    pub(crate) id: i32,
    pub(crate) name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct StructureCompetition {
    pub(crate) id: i32,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct CompetitionStructure {
    pub(crate) id: i32,
    pub(crate) events: Vec<CompetitionEvent>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct CompetitionEvent {
    pub(crate) teams: Vec<CompetitionTeam>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct CompetitionTeam {
    pub(crate) id: i32,
    pub(crate) institution_name: String,
    pub(crate) institution_short_name: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct InstitutionStructure {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) short_name: Option<String>,
    pub(crate) competitions: Vec<StructureCompetition>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct TeamStructure {
    pub(crate) id: i32,
    pub(crate) competitions: Vec<StructureCompetition>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct FilterSummary {
    pub(crate) competitions: usize,
    pub(crate) events: usize,
    pub(crate) teams: usize,
}

#[derive(Clone, Debug)]
pub(crate) struct HomeFilterContext {
    pub(crate) current_path: String,
    pub(crate) request_count_before_last_interaction: usize,
    organizer_options: Vec<EntityOption>,
    competition_options: Vec<EntityOption>,
    institution_options: Vec<EntityOption>,
    team_options: Vec<EntityOption>,
    competition_options_by_organizer: BTreeMap<i32, Vec<EntityOption>>,
    competition_structures: Vec<CompetitionStructure>,
    institution_structures: Vec<InstitutionStructure>,
    team_structures: Vec<TeamStructure>,
    selected_organizer: Option<EntityOption>,
    selected_competition: Option<EntityOption>,
    selected_institution: Option<EntityOption>,
    selected_team: Option<EntityOption>,
}

impl Default for HomeFilterContext {
    fn default() -> Self {
        Self {
            current_path: "/".to_owned(),
            request_count_before_last_interaction: 0,
            organizer_options: Vec::new(),
            competition_options: Vec::new(),
            institution_options: Vec::new(),
            team_options: Vec::new(),
            competition_options_by_organizer: BTreeMap::new(),
            competition_structures: Vec::new(),
            institution_structures: Vec::new(),
            team_structures: Vec::new(),
            selected_organizer: None,
            selected_competition: None,
            selected_institution: None,
            selected_team: None,
        }
    }
}

impl fmt::Debug for ApiWorld {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ApiWorld")
            .field("database_url", &self.database.as_ref().map(|db| &db.url))
            .field("app_ready", &self.app.is_some())
            .field("last_response", &self.last_response)
            .field("request_count", &self.request_count)
            .field("home", &self.home)
            .finish()
    }
}

impl ApiWorld {
    pub(crate) fn api_is_running(&self) -> bool {
        self.database.is_some() && self.app.is_some()
    }

    async fn start_database(&mut self) {
        let container = GenericImage::new("postgres", "16-alpine")
            .with_exposed_port(POSTGRES_PORT.tcp())
            .with_wait_for(WaitFor::message_on_stderr(
                "database system is ready to accept connections",
            ))
            .with_env_var("POSTGRES_DB", POSTGRES_DB)
            .with_env_var("POSTGRES_USER", POSTGRES_USER)
            .with_env_var("POSTGRES_PASSWORD", POSTGRES_PASSWORD)
            .start()
            .await
            .expect("PostgreSQL testcontainer should start");

        let host = container
            .get_host()
            .await
            .expect("PostgreSQL testcontainer should expose host");
        let port = container
            .get_host_port_ipv4(POSTGRES_PORT)
            .await
            .expect("PostgreSQL testcontainer should expose port");
        let url =
            format!("postgres://{POSTGRES_USER}:{POSTGRES_PASSWORD}@{host}:{port}/{POSTGRES_DB}");
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&url)
            .await
            .expect("Cucumber should connect to isolated PostgreSQL");

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Cucumber database migrations should run");

        self.app = Some(routes::create_router().with_state(AppState::new(pool.clone())));
        self.database = Some(TestDatabase {
            container,
            pool,
            url,
        });
    }

    async fn stop_database(&mut self) {
        self.app.take();

        if let Some(database) = self.database.take() {
            database.pool.close().await;

            if let Err(error) = database.container.stop_with_timeout(Some(5)).await {
                eprintln!("failed to stop PostgreSQL testcontainer cleanly: {error}");
            }
        }
    }

    pub(crate) async fn get(&mut self, path: &str) {
        let app = self
            .app
            .as_ref()
            .expect("backend API should be initialized before requests")
            .clone();
        self.request_count += 1;

        let response = app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(path)
                    .body(Body::empty())
                    .expect("request should be valid"),
            )
            .await
            .expect("request should be handled by backend router");
        let status = response.status();
        let body = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("response body should be readable");

        self.last_response = Some(TestResponse {
            status,
            body: String::from_utf8(body.to_vec()).expect("response body should be UTF-8"),
        });
    }

    pub(crate) async fn get_json<T>(&mut self, path: &str) -> T
    where
        T: DeserializeOwned,
    {
        self.get(path).await;

        let response = self
            .last_response
            .as_ref()
            .expect("a response should have been captured");

        assert_eq!(response.status, StatusCode::OK);

        serde_json::from_str(&response.body).expect("response should match the expected JSON shape")
    }

    pub(crate) async fn load_home_filter_context(&mut self) {
        let organizer_options: Vec<EntityOption> = self.get_json("/organizers/options").await;
        let competition_options: Vec<EntityOption> = self.get_json("/competitions/options").await;
        let institution_options: Vec<EntityOption> = self.get_json("/institutions/options").await;
        let team_options: Vec<EntityOption> = self.get_json("/teams/options").await;
        let mut competition_options_by_organizer = BTreeMap::new();

        for organizer in &organizer_options {
            competition_options_by_organizer.insert(
                organizer.id,
                self.get_json(&format!(
                    "/competitions/options?organizer_ids={}",
                    organizer.id
                ))
                .await,
            );
        }

        let competition_structures: Vec<CompetitionStructure> = self
            .get_json(&format!(
                "/competitions/structures?competition_ids={}",
                option_ids_csv(&competition_options)
            ))
            .await;
        let institution_structures: Vec<InstitutionStructure> = self
            .get_json(&format!(
                "/institutions/structures?institution_ids={}",
                option_ids_csv(&institution_options)
            ))
            .await;
        let team_structures: Vec<TeamStructure> = self
            .get_json(&format!(
                "/teams/structures?team_ids={}",
                option_ids_csv(&team_options)
            ))
            .await;

        self.home = HomeFilterContext::new(
            organizer_options,
            competition_options,
            institution_options,
            team_options,
            competition_options_by_organizer,
            competition_structures,
            institution_structures,
            team_structures,
        );
    }
}

impl HomeFilterContext {
    fn new(
        organizer_options: Vec<EntityOption>,
        competition_options: Vec<EntityOption>,
        institution_options: Vec<EntityOption>,
        team_options: Vec<EntityOption>,
        competition_options_by_organizer: BTreeMap<i32, Vec<EntityOption>>,
        competition_structures: Vec<CompetitionStructure>,
        institution_structures: Vec<InstitutionStructure>,
        team_structures: Vec<TeamStructure>,
    ) -> Self {
        Self {
            current_path: "/".to_owned(),
            request_count_before_last_interaction: 0,
            organizer_options,
            competition_options,
            institution_options,
            team_options,
            competition_options_by_organizer,
            competition_structures,
            institution_structures,
            team_structures,
            selected_organizer: None,
            selected_competition: None,
            selected_institution: None,
            selected_team: None,
        }
    }

    pub(crate) fn open_home(&mut self) {
        self.current_path = "/".to_owned();
        self.selected_organizer = None;
        self.selected_competition = None;
        self.selected_institution = None;
        self.selected_team = None;
    }

    pub(crate) fn organizer_exists(&self, name: &str) -> bool {
        self.organizer_options
            .iter()
            .any(|option| option.name == name)
    }

    pub(crate) fn competition_exists(&self, name: &str) -> bool {
        self.competition_options
            .iter()
            .any(|option| option.name == name)
    }

    pub(crate) fn institution_exists(&self, name: &str) -> bool {
        self.institution_options
            .iter()
            .any(|option| option.name == name)
    }

    pub(crate) fn team_exists(&self, name: &str) -> bool {
        self.team_options.iter().any(|option| option.name == name)
    }

    pub(crate) fn select_organizer(&mut self, name: &str) {
        self.selected_organizer = Some(self.find_option(&self.organizer_options, name));
        self.selected_competition = None;
        self.selected_institution = None;
        self.selected_team = None;
    }

    pub(crate) fn select_competition(&mut self, name: &str) {
        let selected = self.find_option(&self.competition_options_for_selected_organizer(), name);
        self.selected_competition = Some(selected);
        self.selected_institution = None;
        self.selected_team = None;
    }

    pub(crate) fn select_institution(&mut self, name: &str) {
        let selected = self.find_option(&self.institution_options_for_selected_competition(), name);
        self.selected_institution = Some(selected);
        self.selected_team = None;
    }

    pub(crate) fn select_team(&mut self, name: &str) {
        let selected = self.find_option(&self.team_options_for_selected_filters(), name);
        self.selected_team = Some(selected);
    }

    pub(crate) fn apply_filters(&mut self) {
        let mut params = Vec::new();

        if let Some(organizer) = &self.selected_organizer {
            params.push(format!("organizer={}", organizer.id));
        }

        if let Some(competition) = &self.selected_competition {
            params.push(format!("competition={}", competition.id));
        }

        if let Some(institution) = &self.selected_institution {
            params.push(format!("institution={}", institution.id));
        }

        if let Some(team) = &self.selected_team {
            params.push(format!("team={}", team.id));
        }

        self.current_path = if params.is_empty() {
            "/".to_owned()
        } else {
            format!("/?{}", params.join("&"))
        };
    }

    pub(crate) fn competition_option_names(&self) -> Vec<String> {
        option_names(&self.competition_options_for_selected_organizer())
    }

    pub(crate) fn institution_option_names(&self) -> Vec<String> {
        option_names(&self.institution_options_for_selected_competition())
    }

    pub(crate) fn team_option_names(&self) -> Vec<String> {
        option_names(&self.team_options_for_selected_filters())
    }

    pub(crate) fn summary(&self) -> FilterSummary {
        let selected_organizer = self.selected_organizer.as_ref().map(|option| option.id);
        let selected_competition = self.selected_competition.as_ref().map(|option| option.id);
        let selected_institution = self.selected_institution.as_ref().map(|option| option.id);
        let selected_team = self.selected_team.as_ref().map(|option| option.id);
        let competition_ids_for_organizer =
            selected_organizer.map(|_| id_set(&self.competition_options_for_selected_organizer()));
        let competition_ids_for_institution = selected_institution.map(|_| {
            self.selected_institution_structure()
                .map(|institution| {
                    institution
                        .competitions
                        .iter()
                        .map(|competition| competition.id)
                        .collect::<BTreeSet<_>>()
                })
                .unwrap_or_default()
        });
        let competition_ids_for_team = selected_team.map(|_| {
            self.selected_team_structure()
                .map(|team| {
                    team.competitions
                        .iter()
                        .map(|competition| competition.id)
                        .collect::<BTreeSet<_>>()
                })
                .unwrap_or_default()
        });
        let has_active_filters = selected_organizer.is_some()
            || selected_competition.is_some()
            || selected_institution.is_some()
            || selected_team.is_some();
        let selected_institution_structure = self.selected_institution_structure();
        let mut summary = FilterSummary::default();
        let mut team_ids = BTreeSet::new();

        for competition in &self.competition_structures {
            if competition_ids_for_organizer
                .as_ref()
                .is_some_and(|ids| !ids.contains(&competition.id))
            {
                continue;
            }

            if selected_competition.is_some_and(|id| competition.id != id) {
                continue;
            }

            if competition_ids_for_institution
                .as_ref()
                .is_some_and(|ids| !ids.contains(&competition.id))
            {
                continue;
            }

            if competition_ids_for_team
                .as_ref()
                .is_some_and(|ids| !ids.contains(&competition.id))
            {
                continue;
            }

            let mut competition_event_count = 0;

            for event in &competition.events {
                let event_team_ids = event
                    .teams
                    .iter()
                    .filter(|team| {
                        selected_institution_structure
                            .is_none_or(|institution| team_matches_institution(team, institution))
                            && selected_team.is_none_or(|id| team.id == id)
                    })
                    .map(|team| team.id)
                    .collect::<Vec<_>>();

                if selected_institution.is_some() || selected_team.is_some() {
                    if event_team_ids.is_empty() {
                        continue;
                    }
                }

                competition_event_count += 1;
                team_ids.extend(event_team_ids);
            }

            if has_active_filters && competition_event_count == 0 {
                continue;
            }

            summary.competitions += 1;
            summary.events += competition_event_count;
        }

        summary.teams = team_ids.len();
        summary
    }

    fn find_option(&self, options: &[EntityOption], name: &str) -> EntityOption {
        options
            .iter()
            .find(|option| option.name == name)
            .cloned()
            .unwrap_or_else(|| panic!("expected option named {name}; options were {options:?}"))
    }

    fn competition_options_for_selected_organizer(&self) -> Vec<EntityOption> {
        let Some(organizer) = &self.selected_organizer else {
            return Vec::new();
        };
        let ids = self
            .competition_options_by_organizer
            .get(&organizer.id)
            .cloned()
            .unwrap_or_default();

        sort_options(ids)
    }

    fn institution_options_for_selected_competition(&self) -> Vec<EntityOption> {
        let Some(competition) = &self.selected_competition else {
            return Vec::new();
        };
        let ids = self
            .institution_structures
            .iter()
            .filter(|institution| {
                institution
                    .competitions
                    .iter()
                    .any(|item| item.id == competition.id)
            })
            .map(|institution| institution.id)
            .collect::<BTreeSet<_>>();

        self.options_by_ids(&self.institution_options, &ids)
    }

    fn team_options_for_selected_filters(&self) -> Vec<EntityOption> {
        let (Some(competition), Some(institution)) = (
            &self.selected_competition,
            self.selected_institution_structure(),
        ) else {
            return Vec::new();
        };
        let ids = self
            .competition_structures
            .iter()
            .find(|structure| structure.id == competition.id)
            .map(|structure| {
                structure
                    .events
                    .iter()
                    .flat_map(|event| &event.teams)
                    .filter(|team| team_matches_institution(team, institution))
                    .map(|team| team.id)
                    .collect::<BTreeSet<_>>()
            })
            .unwrap_or_default();

        self.options_by_ids(&self.team_options, &ids)
    }

    fn selected_institution_structure(&self) -> Option<&InstitutionStructure> {
        let selected = self.selected_institution.as_ref()?;
        self.institution_structures
            .iter()
            .find(|institution| institution.id == selected.id)
    }

    fn selected_team_structure(&self) -> Option<&TeamStructure> {
        let selected = self.selected_team.as_ref()?;
        self.team_structures
            .iter()
            .find(|team| team.id == selected.id)
    }

    fn options_by_ids(&self, options: &[EntityOption], ids: &BTreeSet<i32>) -> Vec<EntityOption> {
        let mut filtered = options
            .iter()
            .filter(|option| ids.contains(&option.id))
            .cloned()
            .collect::<Vec<_>>();
        filtered.sort_by(|left, right| left.name.cmp(&right.name));
        filtered
    }
}

fn option_ids_csv(options: &[EntityOption]) -> String {
    options
        .iter()
        .map(|option| option.id.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn option_names(options: &[EntityOption]) -> Vec<String> {
    options.iter().map(|option| option.name.clone()).collect()
}

fn sort_options(mut options: Vec<EntityOption>) -> Vec<EntityOption> {
    options.sort_by(|left, right| left.name.cmp(&right.name));
    options
}

fn id_set(options: &[EntityOption]) -> BTreeSet<i32> {
    options.iter().map(|option| option.id).collect()
}

fn team_matches_institution(team: &CompetitionTeam, institution: &InstitutionStructure) -> bool {
    team.institution_name == institution.name
        || institution
            .short_name
            .as_ref()
            .is_some_and(|short_name| team.institution_short_name.as_ref() == Some(short_name))
}

#[tokio::main]
async fn main() {
    ApiWorld::cucumber()
        .before(|_, _, _, world| {
            async move {
                world.start_database().await;
            }
            .boxed_local()
        })
        .after(|_, _, _, _, world| {
            async move {
                if let Some(world) = world {
                    world.stop_database().await;
                }
            }
            .boxed_local()
        })
        .run_and_exit("tests/features")
        .await;
}
