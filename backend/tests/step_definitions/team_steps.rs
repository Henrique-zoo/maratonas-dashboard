//! # `cucumber::step_definitions::team_steps`
//!
//! ## Responsabilidade
//! Implementa os passos de existência e seleção de equipe na cascata de filtros.
//!
//! ## Lógica de Implementação
//! Opera sobre `ApiWorld::home`, previamente carregado pela consulta inicial do
//! painel. Os passos de existência verificam a fixture pela API; não inserem dados.
//! As seleções apenas alteram o contexto local e registram o contador de requisições
//! para detectar consultas indevidas. Nomes ausentes e expectativas divergentes
//! produzem panic, registrado pelo Cucumber como falha do step.
//!
//! ## Funções
//! - `team_exists`: Confirma que a equipe já existe nos dados carregados para o contexto de filtros.
//! - `select_team`: Captura o contador de requisições e seleciona a equipe entre as opções atualmente disponíveis.
//!
//! ## Tipos
//! Reutiliza `ApiWorld` e seu contexto de filtros.

use cucumber::{given, when};

use crate::ApiWorld;

/// Confirma que a equipe já existe nos dados carregados para o contexto de filtros.
#[given(regex = r#"^a team named "([^"]+)" exists$"#)]
fn team_exists(world: &mut ApiWorld, name: String) {
    assert!(
        world.home.team_exists(&name),
        "expected team {name} to exist in the home filter context",
    );
}

/// Captura o contador de requisições e seleciona a equipe entre as opções atualmente disponíveis.
#[when(regex = r#"^I select "([^"]+)" in the team filter$"#)]
fn select_team(world: &mut ApiWorld, name: String) {
    world.home.request_count_before_last_interaction = world.request_count;
    world.home.select_team(&name);
}
