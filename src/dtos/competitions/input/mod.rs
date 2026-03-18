use serde::Deserialize;

use crate::shared::serde::CsvOptVec;

// For the main competitions endpoint: "/competitions/structures"
#[derive(Deserialize)]
pub struct CompetitionStructuresQuery {
    pub competition_ids: CsvOptVec<i32>,
}

#[derive(Deserialize)]
pub struct CompetitionByYearQuery {
    pub year: Option<i32>,
}
