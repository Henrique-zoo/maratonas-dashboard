use axum::http::StatusCode;
use cucumber::{gherkin::Step, given, then, when};
use serde_json::Value;

use crate::ApiWorld;

use super::assert_table_values;

#[given("the backend API is running with an isolated database")]
async fn backend_api_is_running(world: &mut ApiWorld) {
    assert!(world.api_is_running());
}

#[given("o contexto de filtros do index foi carregado")]
async fn home_filter_context_is_loaded(world: &mut ApiWorld) {
    world.load_home_filter_context().await;
}

#[given("estou na pagina index")]
fn open_home_page(world: &mut ApiWorld) {
    world.home.open_home();
}

#[given(regex = r#"^existe a organizacao "([^"]+)"$"#)]
fn organization_exists(world: &mut ApiWorld, name: String) {
    assert!(
        world.home.organizer_exists(&name),
        "expected organization {name} to exist in the home filter context",
    );
}

#[when(regex = r#"^seleciono "([^"]+)" no filtro de organizacao$"#)]
fn select_organization(world: &mut ApiWorld, name: String) {
    world.home.request_count_before_last_interaction = world.request_count;
    world.home.select_organizer(&name);
}

#[when("clico em Apply Filters")]
fn apply_filters(world: &mut ApiWorld) {
    world.home.apply_filters();
}

#[then(regex = r#"^devo continuar na pagina "([^"]+)"$"#)]
fn should_stay_on_page(world: &mut ApiWorld, expected_path: String) {
    assert_eq!(world.home.current_path, expected_path);
}

#[then("a selecao nao deve enviar request ao backend")]
fn selection_should_not_request_backend(world: &mut ApiWorld) {
    assert_eq!(
        world.request_count, world.home.request_count_before_last_interaction,
        "selecting an option should only mutate the local filter context",
    );
}

#[then(regex = r#"^devo estar na pagina "([^"]+)"$"#)]
fn should_be_on_page(world: &mut ApiWorld, expected_path: String) {
    assert_eq!(world.home.current_path, expected_path);
}

#[then("as opcoes do filtro de competicoes devem ser:")]
fn competition_filter_options(world: &mut ApiWorld, #[step] step: &Step) {
    assert_table_values(step, world.home.competition_option_names());
}

#[then(expr = "devo ver {int} competicoes no resumo dos filtros")]
fn summary_competitions(world: &mut ApiWorld, expected: usize) {
    assert_eq!(world.home.summary().competitions, expected);
}

#[then(expr = "devo ver {int} eventos no resumo dos filtros")]
fn summary_events(world: &mut ApiWorld, expected: usize) {
    assert_eq!(world.home.summary().events, expected);
}

#[then(expr = "devo ver {int} times no resumo dos filtros")]
fn summary_teams(world: &mut ApiWorld, expected: usize) {
    assert_eq!(world.home.summary().teams, expected);
}

#[when(regex = r#"^I request GET "([^"]+)"$"#)]
async fn request_get(world: &mut ApiWorld, path: String) {
    world.get(&path).await;
}

#[then(expr = "the response status should be {int}")]
async fn response_status(world: &mut ApiWorld, expected_status: u16) {
    let response = world
        .last_response
        .as_ref()
        .expect("a response should have been captured");

    assert_eq!(
        response.status,
        StatusCode::from_u16(expected_status).expect("status should be valid"),
    );
}

#[then(regex = r#"^the response JSON should include an organizer named "([^"]+)"$"#)]
async fn response_json_includes_organizer(world: &mut ApiWorld, expected_name: String) {
    let response = world
        .last_response
        .as_ref()
        .expect("a response should have been captured");
    let payload: Value = serde_json::from_str(&response.body).expect("response should be JSON");
    let organizers = payload.as_array().expect("response should be a JSON array");

    assert!(
        organizers
            .iter()
            .any(|organizer| organizer.get("name").and_then(Value::as_str) == Some(&expected_name)),
        "expected organizer named {expected_name} in response: {}",
        response.body,
    );
}
