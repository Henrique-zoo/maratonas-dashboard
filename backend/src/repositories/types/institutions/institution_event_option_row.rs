//! Projeção dos eventos históricos disponíveis para uma instituição.

use sqlx::prelude::FromRow;

#[derive(Clone, FromRow)]
pub struct InstitutionEventOptionRow {
    pub event_id: i32,
    pub event_name: String,
    pub competition_id: i32,
    pub competition_name: String,
    pub years: Vec<i32>,
}
