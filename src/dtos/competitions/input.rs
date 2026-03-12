use serde::Deserialize;

use crate::shared::serde::CsvOptVec;

#[derive(Deserialize)]
pub struct CompetitionStructuresQuery {
    pub competition_ids: CsvOptVec<i32>,
}
