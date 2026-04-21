use serde::Serialize;

use crate::repositories::types::IdNameRow;

// Simple DTO for filter options (id + name).
#[derive(Debug, Serialize)]
pub struct Filter {
    pub id: i32,
    pub name: String,
}

// Converts a raw repository row into the filter payload.
impl From<IdNameRow> for Filter {
    fn from(row: IdNameRow) -> Self {
        Self {
            id: row.id,
            name: row.name,
        }
    }
}
