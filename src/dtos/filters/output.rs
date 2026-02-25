use serde::Serialize;

use crate::repositories::types::IdNameRow;

#[derive(Serialize)]
pub struct FilterDto {
    pub id: i32,
    pub name: String,
}

impl From<IdNameRow> for FilterDto {
    fn from(row: IdNameRow) -> Self {
        Self {
            id: row.id,
            name: row.name
        }
    }
}