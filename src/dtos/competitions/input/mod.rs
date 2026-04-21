use serde::Deserialize;

use crate::shared::serde::CsvOptVec;

// For the main competitions endpoint: "/competitions/structures"
#[derive(Debug, Deserialize)]
pub struct CompetitionStructuresQuery {
    pub competition_ids: CsvOptVec<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CompetitionByYearQuery {
    pub year: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct TeamCompetitionStructureQuery {
    pub team_id: i32,
    pub competition_id: i32,
}
