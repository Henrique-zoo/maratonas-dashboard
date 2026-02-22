use serde::{Serialize, Deserialize};
use crate::shared::LocationType;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Location {
    pub id: i32,
    pub parent_id: Option<i32>,
    pub location_type: LocationType,
    pub name: String,
}