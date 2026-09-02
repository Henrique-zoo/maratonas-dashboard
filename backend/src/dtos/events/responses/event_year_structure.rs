//! Contrato público da identidade e das ocorrências anuais de um evento.

use chrono::NaiveDate;
use serde::Serialize;

use crate::{
    repositories::types::events::EventYearStructureRow,
    shared::types::{LocationType, Scope},
};

#[derive(Debug, Serialize)]
pub struct EventCompetition {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct EventInstance {
    pub id: i32,
    pub date: NaiveDate,
    pub location: String,
}

/// Evento lógico acompanhado das ocorrências concretas do ano selecionado.
#[derive(Debug, Serialize)]
pub struct EventYearStructure {
    pub id: i32,
    pub name: String,
    pub level: Option<u32>,
    pub scope: Scope,
    pub competition: EventCompetition,
    pub year: u32,
    pub years: Vec<u32>,
    pub location_types: Vec<LocationType>,
    pub instances: Vec<EventInstance>,
}

impl EventYearStructure {
    pub fn from_rows(rows: Vec<EventYearStructureRow>) -> Option<Self> {
        let first = rows.first()?.clone();

        Some(Self {
            id: first.event_id,
            name: first.event_name,
            level: first.event_level.map(|level| level as u32),
            scope: first.event_scope,
            competition: EventCompetition {
                id: first.competition_id,
                name: first.competition_name,
            },
            year: first.selected_year as u32,
            years: first
                .available_years
                .into_iter()
                .map(|year| year as u32)
                .collect(),
            location_types: first.event_location_types,
            instances: rows
                .into_iter()
                .map(|row| EventInstance {
                    id: row.event_instance_id,
                    date: row.event_date,
                    location: row.event_location,
                })
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn structure_preserves_all_instances_in_the_selected_year() {
        let row = |instance_id, day| EventYearStructureRow {
            event_id: 10,
            event_name: "Regional".to_string(),
            event_level: Some(1),
            event_scope: Scope::Regional,
            competition_id: 1,
            competition_name: "ICPC".to_string(),
            selected_year: 2025,
            available_years: vec![2024, 2025],
            event_location_types: vec![LocationType::Country, LocationType::City],
            event_instance_id: instance_id,
            event_date: NaiveDate::from_ymd_opt(2025, 5, day).unwrap(),
            event_location: "Brazil, Sao Paulo".to_string(),
        };

        let structure = EventYearStructure::from_rows(vec![row(100, 10), row(101, 11)]).unwrap();

        assert_eq!(structure.year, 2025);
        assert_eq!(structure.years, vec![2024, 2025]);
        assert_eq!(structure.instances.len(), 2);
    }
}
