use serde::Deserialize;

use crate::shared::serde::CsvOptVec;

#[derive(Deserialize)]
pub struct InstitutionStructuresQuery {
    pub intitution_ids: CsvOptVec<i32>,
}
