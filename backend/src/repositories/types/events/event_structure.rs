//! Projeções SQL usadas para montar a estrutura autoritativa de um evento.

use chrono::NaiveDate;
use sqlx::prelude::FromRow;

use crate::shared::types::{LocationType, Scope};

/// Uma ocorrência concreta de um evento lógico no ano selecionado.
#[derive(Clone, FromRow)]
pub struct EventYearStructureRow {
    pub event_id: i32,
    pub event_name: String,
    pub event_level: Option<i32>,
    pub event_scope: Scope,
    pub competition_id: i32,
    pub competition_name: String,
    pub selected_year: i32,
    pub available_years: Vec<i32>,
    pub event_location_types: Vec<LocationType>,
    pub event_instance_id: i32,
    pub event_date: NaiveDate,
    pub event_location: String,
}
