use serde::Serialize;

use crate::repositories::types::events::EventYearStatsRow;

#[derive(Debug, Serialize)]
pub struct EventYearStats {
    pub total_institutions: u32,
    pub total_teams: u32,
    pub total_participants: u32,
    pub female_participants: u32,
}

impl From<EventYearStatsRow> for EventYearStats {
    fn from(value: EventYearStatsRow) -> Self {
        Self {
            total_institutions: value.total_institutions as u32,
            total_teams: value.total_teams as u32,
            total_participants: value.total_participants as u32,
            female_participants: value.female_participants as u32,
        }
    }
}
