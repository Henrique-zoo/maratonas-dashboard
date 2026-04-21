use serde::Deserialize;

use crate::shared::serde::CsvOptVec;

#[derive(Debug, Deserialize)]
pub struct OptionsQuery {
    pub organizer_ids: CsvOptVec<i32>,
}

#[derive(Debug, Deserialize)]
pub struct StructuresQuery {
    pub competition_ids: CsvOptVec<i32>,
}
