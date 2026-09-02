//! # `backend::repositories::types::teams::team_option_row`
//!
//! Projeção de uma equipe selecionável acompanhada de sua instituição.

use sqlx::FromRow;

/// Linha usada para montar uma opção de equipe sem perder o contexto
/// institucional necessário para desambiguar nomes repetidos.
#[derive(Clone, Debug, FromRow)]
pub struct TeamOptionRow {
    pub id: i32,
    pub name: String,
    pub institution_id: i32,
    pub institution_name: String,
    pub institution_short_name: Option<String>,
}
