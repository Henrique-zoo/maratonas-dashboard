//! Opção de evento disponível para a análise histórica de uma instituição.

use serde::Serialize;

use crate::repositories::types::institutions::InstitutionEventOptionRow;

#[derive(Debug, Serialize)]
pub struct InstitutionEventOption {
    pub id: i32,
    pub name: String,
    pub competition_id: i32,
    pub competition_name: String,
    pub years: Vec<u32>,
}

impl From<InstitutionEventOptionRow> for InstitutionEventOption {
    fn from(value: InstitutionEventOptionRow) -> Self {
        Self {
            id: value.event_id,
            name: value.event_name,
            competition_id: value.competition_id,
            competition_name: value.competition_name,
            years: value.years.into_iter().map(|year| year as u32).collect(),
        }
    }
}
