use serde::Serialize;

use crate::repositories::types::IdNameRow;

#[derive(Serialize)]
pub struct Filter {
    pub id: i32,
    pub name: String,
}

impl From<IdNameRow> for Filter {
    fn from(row: IdNameRow) -> Self {
        Self {
            id: row.id,
            name: row.name
        }
    }
}