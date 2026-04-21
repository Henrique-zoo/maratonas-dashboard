use serde::Serialize;

use super::EventSubStructure;

#[derive(Default, Debug, Serialize)]
pub struct CompetitionYearStructure {
    pub events: Vec<EventSubStructure>,
}
