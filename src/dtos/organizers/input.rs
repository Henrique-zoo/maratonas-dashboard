use serde::Deserialize;

use crate::shared::serde::CsvOptVec;

#[derive(Deserialize)]
pub struct OrganizerStructuresQuery {
    pub organizer_ids: CsvOptVec<i32>,
}
