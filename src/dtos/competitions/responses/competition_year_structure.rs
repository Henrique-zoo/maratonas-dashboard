use indexmap::IndexMap;
use serde::Serialize;

use crate::shared::types::LocationType;

use super::EventSubStructure;

#[derive(Debug, Serialize)]
pub struct CompetitionYearStructure {
    pub location_types: Vec<LocationType>,
    pub events: Vec<EventSubStructure>,
}

#[derive(Default, Debug)]
pub struct TempCompetitionYearStructure {
    pub location_types: Vec<LocationType>,
    pub events: IndexMap<i32, super::TempEventSubStructure>,
}

impl From<TempCompetitionYearStructure> for CompetitionYearStructure {
    fn from(value: TempCompetitionYearStructure) -> Self {
        let mut location_types = value.location_types;
        location_types.sort();
        Self {
            location_types,
            events: value
                .events
                .into_values()
                .map(EventSubStructure::from)
                .collect(),
        }
    }
}

impl TempCompetitionYearStructure {
    pub fn update(&mut self, location_types: Vec<LocationType>) {
        self.location_types = location_types;
    }
}
