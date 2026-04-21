use serde::Deserialize;

use crate::shared::serde::CsvOptVec;

#[derive(Deserialize)]
pub struct StructuresQuery {
    pub team_ids: CsvOptVec<i32>,
}
