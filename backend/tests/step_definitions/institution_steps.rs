//! # `cucumber::step_definitions::institution_steps`
//!
//! ## Responsabilidade
//! Implementa os passos de existência e seleção de instituição na cascata de filtros.
//!
//! ## Lógica de Implementação
//! Opera sobre `ApiWorld::home`, previamente carregado pela consulta inicial do
//! painel. Os passos de existência verificam a fixture pela API; não inserem dados.
//! As seleções apenas alteram o contexto local e registram o contador de requisições
//! para detectar consultas indevidas. Nomes ausentes e expectativas divergentes
//! produzem panic, registrado pelo Cucumber como falha do step.
//!
//! ## Funções
//! - `institution_exists`: Confirma que a instituição já existe nos dados carregados para o contexto de filtros.
//! - `select_institution`: Captura o contador de requisições e seleciona uma instituição da competição atual.
//! - `team_filter_options`: Compara as equipes disponíveis com a tabela ordenada de coluna única.
//!
//! ## Tipos
//! Reutiliza `ApiWorld` e seu contexto de filtros.

use cucumber::{gherkin::Step, given, then, when};

use crate::ApiWorld;

use super::assert_table_values;

/// Confirma que a instituição já existe nos dados carregados para o contexto de filtros.
#[given(regex = r#"^an institution named "([^"]+)" exists$"#)]
fn institution_exists(world: &mut ApiWorld, name: String) {
    assert!(
        world.home.institution_exists(&name),
        "expected institution {name} to exist in the home filter context",
    );
}

/// Captura o contador de requisições e seleciona uma instituição da competição atual.
#[when(regex = r#"^I select "([^"]+)" in the institution filter$"#)]
fn select_institution(world: &mut ApiWorld, name: String) {
    world.home.request_count_before_last_interaction = world.request_count;
    world.home.select_institution(&name);
}

/// Compara as equipes disponíveis com a tabela ordenada de coluna única.
#[then("the team filter options should be:")]
fn team_filter_options(world: &mut ApiWorld, #[step] step: &Step) {
    assert_table_values(step, world.home.team_option_names());
}
