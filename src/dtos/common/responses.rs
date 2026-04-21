use serde::Serialize;

use crate::repositories::types::IdNameRow;

#[derive(Debug, Serialize)]
pub struct OptionItem {
    pub id: i32,
    pub name: String,
}

impl From<IdNameRow> for OptionItem {
    fn from(row: IdNameRow) -> Self {
        Self {
            id: row.id,
            name: row.name,
        }
    }
}
