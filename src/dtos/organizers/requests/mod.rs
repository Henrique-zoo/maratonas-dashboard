use serde::Deserialize;

use crate::shared::serde::CsvOptVec;

#[derive(Deserialize)]
pub struct StructuresQuery {
    pub organizer_ids: CsvOptVec<i32>,
}
