//! # `cucumber::step_definitions::organization_steps`
//!
//! ## Responsabilidade
//! Traduz ações de consulta analítica e de seleção de organizador para o World.
//!
//! ## Lógica de Implementação
//! Os steps de consulta montam URIs e delegam ao router real por `ApiWorld::get`.
//! Os steps do painel carregam dados da API uma vez e manipulam `HomeFilterContext`
//! em memória; verificam endereço e contagem de requisições, sem executar browser.
//! As expectativas de domínio sobre respostas ficam em `domain_response_steps`.
//!
//! ## Funções
//! - `view_competition_dashboard`: prepara o contexto de filtros.
//! - `select_organization`, `apply_filters`: alteram a seleção e o endereço local.
//! - Steps `request_*`, `list_*` e `group_*`: consultam endpoints analíticos.
//! - Steps `dashboard_*` e `filter_*`: verificam a interação simulada.
//!
//! ## Tipos
//! Reutiliza `ApiWorld` e tabelas Gherkin `Step`.

use cucumber::{gherkin::Step, given, then, when};

use crate::ApiWorld;

use super::assert_table_values;

/// Encaminha GET ao helper do World, que atualiza o contador e a última resposta.
async fn request(world: &mut ApiWorld, path: String) {
    world.get(&path).await;
}

/// Carrega opções e estruturas pela API e abre o contexto local de filtros sem seleções.
#[given("I am viewing the competition dashboard")]
async fn view_competition_dashboard(world: &mut ApiWorld) {
    world.load_home_filter_context().await;
    world.home.open_home();
}

/// Registra o contador de requisições e seleciona o organizador, limpando filtros dependentes.
#[when(regex = r#"^I select "([^"]+)" in the organizer filter$"#)]
fn select_organization(world: &mut ApiWorld, name: String) {
    world.home.request_count_before_last_interaction = world.request_count;
    world.home.select_organizer(&name);
}

/// Atualiza o endereço simulado com os filtros selecionados, sem realizar outra requisição.
#[when("I apply the filters")]
fn apply_filters(world: &mut ApiWorld) {
    world.home.apply_filters();
}

/// Verifica que o endereço simulado continua na raiz antes da aplicação dos filtros.
#[then("the dashboard should not navigate")]
fn dashboard_should_not_navigate(world: &mut ApiWorld) {
    assert_eq!(world.home.current_path, "/");
}

/// Verifica que a última seleção não aumentou o contador de requisições do World.
#[then("the dashboard should wait for me to apply the filters before refreshing")]
fn dashboard_should_wait_before_refreshing(world: &mut ApiWorld) {
    assert_eq!(
        world.request_count, world.home.request_count_before_last_interaction,
        "selecting an option should only mutate the local filter context",
    );
}

/// Compara o endereço simulado com a query string esperada após aplicar os filtros.
#[then(regex = r#"^the page address should represent the selected filters as "([^"]+)"$"#)]
fn selected_filters_should_be_in_page_address(world: &mut ApiWorld, expected_path: String) {
    assert_eq!(world.home.current_path, expected_path);
}

/// Compara os nomes das competições disponíveis com a tabela de coluna única, em ordem.
#[then("the competition filter options should be:")]
fn competition_filter_options(world: &mut ApiWorld, #[step] step: &Step) {
    assert_table_values(step, world.home.competition_option_names());
}

/// Compara os totais locais de competições, eventos e equipes com os valores do cenário.
#[then(
    regex = r"^the filter summary should show (\d+) competitions?, (\d+) events?, and (\d+) teams?$"
)]
fn filter_summary_should_equal(
    world: &mut ApiWorld,
    expected_competitions: usize,
    expected_events: usize,
    expected_teams: usize,
) {
    let summary = world.home.summary();
    assert_eq!(summary.competitions, expected_competitions);
    assert_eq!(summary.events, expected_events);
    assert_eq!(summary.teams, expected_teams);
}

/// Consulta as estatísticas da competição no ano informado e armazena a resposta.
#[when(expr = "I request competition {int} statistics for {int}")]
async fn request_competition_statistics(world: &mut ApiWorld, competition_id: i32, year: i32) {
    request(
        world,
        format!("/competitions/{competition_id}/stats?year={year}"),
    )
    .await;
}

/// Consulta as estatísticas do evento no ano informado e armazena a resposta.
#[when(expr = "I request event {int} statistics for {int}")]
async fn request_event_statistics(world: &mut ApiWorld, event_id: i32, year: i32) {
    request(world, format!("/events/{event_id}/stats?year={year}")).await;
}

/// Consulta as estatísticas anuais da competição agrupadas por `Country`.
#[when(expr = "I group competition {int} statistics by country for {int}")]
async fn group_competition_statistics_by_country(
    world: &mut ApiWorld,
    competition_id: i32,
    year: i32,
) {
    request(
        world,
        format!("/competitions/{competition_id}/location-stats?location_type=Country&year={year}"),
    )
    .await;
}

/// Consulta as estatísticas anuais do evento agrupadas por `Country`.
#[when(expr = "I group event {int} statistics by country for {int}")]
async fn group_event_statistics_by_country(world: &mut ApiWorld, event_id: i32, year: i32) {
    request(
        world,
        format!("/events/{event_id}/location-stats?location_type=Country&year={year}"),
    )
    .await;
}

/// Consulta as opções de eventos com histórico de participação da instituição.
#[when(expr = "I request the event history for institution {int}")]
async fn request_institution_event_history(world: &mut ApiWorld, institution_id: i32) {
    request(
        world,
        format!("/institutions/{institution_id}/events/options"),
    )
    .await;
}

/// Consulta o desempenho institucional no evento entre os anos inicial e final informados.
#[when(expr = "I request institution {int}'s performance in event {int} from {int} through {int}")]
async fn request_institution_performance(
    world: &mut ApiWorld,
    institution_id: i32,
    event_id: i32,
    start_year: i32,
    end_year: i32,
) {
    request(
        world,
        format!(
            "/institutions/{institution_id}/events/{event_id}/performance?start_year={start_year}&end_year={end_year}"
        ),
    )
    .await;
}

/// Consulta o catálogo de opções de organizadores.
#[when("I list the available organizers")]
async fn list_organizers(world: &mut ApiWorld) {
    request(world, "/organizers/options".to_owned()).await;
}

/// Consulta opções de competição sem filtro de organizador.
#[when("I list all competitions")]
async fn list_all_competitions(world: &mut ApiWorld) {
    request(world, "/competitions/options".to_owned()).await;
}

/// Consulta opções de competição restritas ao organizador informado.
#[when(expr = "I list competitions organized by organizer {int}")]
async fn list_competitions_by_organizer(world: &mut ApiWorld, organizer_id: i32) {
    request(
        world,
        format!("/competitions/options?organizer_ids={organizer_id}"),
    )
    .await;
}

/// Consulta instituições com participação na competição informada.
#[when(expr = "I list institutions that participated in competition {int}")]
async fn list_institutions_by_competition(world: &mut ApiWorld, competition_id: i32) {
    request(
        world,
        format!("/institutions/options?competition_ids={competition_id}"),
    )
    .await;
}

/// Consulta o catálogo de opções de instituições sem filtro de competição.
#[when("I list all institutions")]
async fn list_all_institutions(world: &mut ApiWorld) {
    request(world, "/institutions/options".to_owned()).await;
}

/// Consulta o catálogo de opções de equipes sem filtros.
#[when("I list all teams")]
async fn list_all_teams(world: &mut ApiWorld) {
    request(world, "/teams/options".to_owned()).await;
}

/// Consulta equipes combinando filtros de instituição e competição.
#[when(expr = "I list teams from institution {int} that participated in competition {int}")]
async fn list_teams_by_institution_and_competition(
    world: &mut ApiWorld,
    institution_id: i32,
    competition_id: i32,
) {
    request(
        world,
        format!("/teams/options?competition_ids={competition_id}&institution_ids={institution_id}"),
    )
    .await;
}

/// Consulta equipes com dois IDs de competição e dois de instituição em parâmetros CSV.
#[when(expr = "I list teams for competitions {int} and {int} and institutions {int} and {int}")]
async fn list_teams_for_multiple_filters(
    world: &mut ApiWorld,
    first_competition_id: i32,
    second_competition_id: i32,
    first_institution_id: i32,
    second_institution_id: i32,
) {
    request(
        world,
        format!(
            "/teams/options?competition_ids={first_competition_id},{second_competition_id}&institution_ids={first_institution_id},{second_institution_id}"
        ),
    )
    .await;
}

/// Consulta opções de equipes filtradas somente pela instituição.
#[when(expr = "I list teams from institution {int}")]
async fn list_teams_by_institution(world: &mut ApiWorld, institution_id: i32) {
    request(
        world,
        format!("/teams/options?institution_ids={institution_id}"),
    )
    .await;
}

/// Consulta a visão atual das competições do organizador informado.
#[when(expr = "I request organizer {int}'s current competition overview")]
async fn request_organizer_overview(world: &mut ApiWorld, organizer_id: i32) {
    request(
        world,
        format!("/organizers/structures?organizer_ids={organizer_id}"),
    )
    .await;
}

/// Consulta a visão do organizador para uma competição e ano específicos.
#[when(expr = "I request competition {int}'s organizer overview for {int}")]
async fn request_organizer_competition_overview(
    world: &mut ApiWorld,
    competition_id: i32,
    year: i32,
) {
    request(
        world,
        format!("/organizers/competitions/{competition_id}/structure?year={year}"),
    )
    .await;
}

/// Consulta a estrutura detalhada atual da competição pelo endpoint de estruturas.
#[when(expr = "I request competition {int}'s latest detailed view")]
async fn request_latest_competition_view(world: &mut ApiWorld, competition_id: i32) {
    request(
        world,
        format!("/competitions/structures?competition_ids={competition_id}"),
    )
    .await;
}

/// Consulta a estrutura detalhada anual da competição.
#[when(expr = "I request competition {int}'s detailed view for {int}")]
async fn request_competition_view(world: &mut ApiWorld, competition_id: i32, year: i32) {
    request(
        world,
        format!("/competitions/{competition_id}/structure?year={year}"),
    )
    .await;
}

/// Consulta o portfólio completo da instituição, sem restringir por competição.
#[when(expr = "I request institution {int}'s complete portfolio")]
async fn request_institution_portfolio(world: &mut ApiWorld, institution_id: i32) {
    request(
        world,
        format!("/institutions/structures?institution_ids={institution_id}"),
    )
    .await;
}

/// Consulta o portfólio completo da equipe, sem restringir por competição.
#[when(expr = "I request team {int}'s complete portfolio")]
async fn request_team_portfolio(world: &mut ApiWorld, team_id: i32) {
    request(world, format!("/teams/structures?team_ids={team_id}")).await;
}

/// Consulta a estrutura de desempenho da equipe em uma competição e ano.
#[when(expr = "I request team {int}'s performance in competition {int} for {int}")]
async fn request_team_performance(
    world: &mut ApiWorld,
    team_id: i32,
    competition_id: i32,
    year: i32,
) {
    request(
        world,
        format!("/teams/{team_id}/competitions/{competition_id}/structure?year={year}"),
    )
    .await;
}

/// Consulta a estrutura do evento usando a seleção de ano padrão da API.
#[when(expr = "I request event {int}'s current view")]
async fn request_current_event_view(world: &mut ApiWorld, event_id: i32) {
    request(world, format!("/events/{event_id}/structure")).await;
}

/// Consulta a estrutura do evento para o ano explicitamente informado.
#[when(expr = "I request event {int}'s view for {int}")]
async fn request_event_view(world: &mut ApiWorld, event_id: i32, year: i32) {
    request(world, format!("/events/{event_id}/structure?year={year}")).await;
}

/// Executa GET no caminho literal do cenário para testar contratos e validações HTTP.
#[when(regex = r#"^a client sends GET "([^"]+)"$"#)]
async fn client_sends_get(world: &mut ApiWorld, path: String) {
    request(world, path).await;
}
