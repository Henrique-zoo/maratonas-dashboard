use serde::Serialize;

use crate::repositories::types::competitions::CompetitionYearStatsRow;

#[derive(Debug, Serialize)]
pub struct CompetitionYearStats {
    pub total_institutions: u32,
    pub total_teams: u32,
    pub total_participants: u32,
    pub female_participants: u32,
}

impl From<CompetitionYearStatsRow> for CompetitionYearStats {
    fn from(value: CompetitionYearStatsRow) -> Self {
        Self {
            total_institutions: value.total_institutions as u32,
            total_teams: value.total_teams as u32,
            total_participants: value.total_participants as u32,
            female_participants: value.female_participants as u32,
        }
    }
}
