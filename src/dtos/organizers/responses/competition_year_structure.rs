use serde::Serialize;

use crate::shared::types::LocationType;

use super::EventSubStructure;

#[derive(Default, Debug, Serialize)]
pub struct CompetitionYearStructure {
    pub location_types: Vec<LocationType>,
    pub events: Vec<EventSubStructure>,
}

impl CompetitionYearStructure {
    pub fn update(&mut self, location_types: Vec<LocationType>) {
        self.location_types = location_types;
    }
}
