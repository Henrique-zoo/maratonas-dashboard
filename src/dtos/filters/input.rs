use serde::Deserialize;

use crate::shared::serde::CsvOptVec;

#[derive(Deserialize)]
pub struct CompetitionOptionsQuery {
    pub organizer_ids: CsvOptVec<i32>
}

#[derive(Deserialize)]
pub struct InstitutionOptionsQuery {
    pub competition_ids: CsvOptVec<i32>
}

#[derive(Deserialize)]
pub struct TeamOptionQuery {
    pub competition_ids: CsvOptVec<i32>,
    pub institution_ids: CsvOptVec<i32>
}