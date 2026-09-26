//! # `cucumber::step_definitions`
//!
//! ## Responsabilidade
//! Registra os módulos de steps e concentra comparações de tabelas usadas nos cenários.
//!
//! ## Lógica de Implementação
//! Os atributos `given`, `when` e `then` conectam frases Gherkin às funções sobre
//! [`crate::ApiWorld`]. Asserções usam panic para que o Cucumber registre falhas.
//! Tabelas de coluna única não possuem cabeçalho; as tabelas de domínio validam
//! seus cabeçalhos em `domain_response_steps`.
//!
//! ## Submódulos
//! - `api_steps`: verificações genéricas do contrato HTTP e do JSON.
//! - `organization_steps`: requisições analíticas e contexto inicial dos filtros.
//! - `competition_steps`, `institution_steps`, `team_steps`: seleção em cascata.
//! - `domain_response_steps`: projeções de respostas em tabelas de domínio.
//! - `membership_steps`: preparação SQL, constraints e mutações concorrentes.
//!
//! ## Funções
//! - [`single_column_table`]: lê os valores esperados sem reordená-los.
//! - [`assert_table_values`]: compara valores e ordem com a tabela Gherkin.
//!
//! ## Tipos
//! Reutiliza `Step` e o `ApiWorld` definido no runner.

use cucumber::gherkin::Step;

pub(crate) mod api_steps;
pub(crate) mod competition_steps;
pub(crate) mod domain_response_steps;
pub(crate) mod institution_steps;
pub(crate) mod membership_steps;
pub(crate) mod organization_steps;
pub(crate) mod team_steps;

/// Extrai uma tabela Gherkin de coluna única preservando a ordem das linhas.
///
/// # Parâmetros
/// - `step`: passo que contém a tabela, sem linha de cabeçalho especial.
///
/// # Retorno
/// Valores da única coluna; todas as linhas participam da comparação.
///
/// # Erros
/// Dispara panic se a tabela estiver ausente ou alguma linha tiver outra quantidade de colunas.
pub(crate) fn single_column_table(step: &Step) -> Vec<String> {
    let table = step
        .table()
        .expect("step should provide a single-column data table");

    table
        .rows
        .iter()
        .map(|row| {
            assert_eq!(row.len(), 1, "expected a single-column data table row");
            row[0].clone()
        })
        .collect()
}

/// Compara uma sequência de valores com a tabela de coluna única de um step.
///
/// # Parâmetros
/// - `step`: tabela com os valores esperados, na ordem de apresentação.
/// - `actual`: valores produzidos pela API ou pelo contexto de filtros.
///
/// # Erros
/// Dispara panic em tabela inválida ou diferença de conteúdo, tamanho ou ordem.
pub(crate) fn assert_table_values(step: &Step, actual: Vec<String>) {
    let expected = single_column_table(step);

    assert_eq!(actual, expected);
}
