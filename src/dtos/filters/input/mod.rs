use serde::Deserialize;

use crate::shared::{serde::CsvOptVec, types::LocationType};

#[derive(Deserialize)]
pub struct CompetitionOptionsQuery {
    pub organizer_ids: CsvOptVec<i32>,
}

#[derive(Deserialize)]
pub struct InstitutionOptionsQuery {
    pub competition_ids: CsvOptVec<i32>,
}

#[derive(Deserialize)]
pub struct TeamOptionQuery {
    pub competition_ids: CsvOptVec<i32>,
    pub institution_ids: CsvOptVec<i32>,
}

#[derive(Deserialize)]
pub struct IdPath {
    pub id: i32,
}

#[derive(Deserialize)]
pub struct LocationYearStatsQuery {
    pub location_type: Option<LocationType>,
    pub year: Option<i32>,
}
