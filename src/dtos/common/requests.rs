use serde::Deserialize;

use crate::shared::types::LocationType;

#[derive(Debug, Deserialize)]
pub struct IdPath {
    pub id: i32,
}

#[derive(Debug, Deserialize)]
pub struct YearQuery {
    pub year: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct LocationYearQuery {
    pub location_type: Option<LocationType>,
    pub year: Option<i32>,
}
