//! # `cucumber::step_definitions::competition_steps`
//!
//! ## Responsabilidade
//! Implementa os passos de existência e seleção de competição na cascata de filtros.
//!
//! ## Lógica de Implementação
//! Opera sobre `ApiWorld::home`, previamente carregado pela consulta inicial do
//! painel. Os passos de existência verificam a fixture pela API; não inserem dados.
//! As seleções apenas alteram o contexto local e registram o contador de requisições
//! para detectar consultas indevidas. Nomes ausentes e expectativas divergentes
//! produzem panic, registrado pelo Cucumber como falha do step.
//!
//! ## Funções
//! - `competition_exists`: Confirma que a competição já existe nos dados carregados para o contexto de filtros.
//! - `select_competition`: Captura o contador de requisições e seleciona a competição entre as opções do organizador.
//! - `institution_filter_options`: Compara as instituições disponíveis com a tabela ordenada de coluna única.
//!
//! ## Tipos
//! Reutiliza `ApiWorld` e seu contexto de filtros.

use cucumber::{gherkin::Step, given, then, when};

use crate::ApiWorld;

use super::assert_table_values;

/// Confirma que a competição já existe nos dados carregados para o contexto de filtros.
#[given(regex = r#"^a competition named "([^"]+)" exists$"#)]
fn competition_exists(world: &mut ApiWorld, name: String) {
    assert!(
        world.home.competition_exists(&name),
        "expected competition {name} to exist in the home filter context",
    );
}

/// Captura o contador de requisições e seleciona a competição entre as opções do organizador.
#[when(regex = r#"^I select "([^"]+)" in the competition filter$"#)]
fn select_competition(world: &mut ApiWorld, name: String) {
    world.home.request_count_before_last_interaction = world.request_count;
    world.home.select_competition(&name);
}

/// Compara as instituições disponíveis com a tabela ordenada de coluna única.
#[then("the institution filter options should be:")]
fn institution_filter_options(world: &mut ApiWorld, #[step] step: &Step) {
    assert_table_values(step, world.home.institution_option_names());
}
