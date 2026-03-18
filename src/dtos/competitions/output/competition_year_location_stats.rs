use serde::Serialize;

use crate::repositories::types::competitions::CompetitionLocationStatsRow;

#[derive(Debug, Serialize)]
pub struct CompetitionYearLocationStats {
    pub id: i32,
    pub name: String,
    pub total_institutions: u32,
    pub total_teams: u32,
    pub total_participants: u32,
    pub female_percentage: f32,
}

impl From<CompetitionLocationStatsRow> for CompetitionYearLocationStats {
    fn from(value: CompetitionLocationStatsRow) -> Self {
        Self {
            id: value.location_id,
            name: value.location_name,
            total_institutions: value.total_institutions as u32,
            total_teams: value.total_teams as u32,
            total_participants: value.total_participants as u32,
            female_percentage: value.female_participants as f32 / value.total_participants as f32,
        }
    }
}
