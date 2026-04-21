mod structures_query;

use serde::Deserialize;

use crate::shared::serde::CsvOptVec;

pub use structures_query::*;

#[derive(Debug, Deserialize)]
pub struct OptionsQuery {
    pub competition_ids: CsvOptVec<i32>,
    pub institution_ids: CsvOptVec<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CompetitionStructurePath {
    pub team_id: i32,
    pub competition_id: i32,
}
