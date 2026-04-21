use chrono::NaiveDate;
use sqlx::prelude::FromRow;

use crate::shared::types::LocationType;

#[derive(FromRow)]
pub struct CompetitionEventsByYearRow {
    pub competition_location_types: Vec<LocationType>,

    pub event_id: i32,
    pub event_name: String,
    pub event_level: Option<i32>,
    pub event_date: NaiveDate,
    pub event_location: String,
    pub event_total_institutions: i32,
    pub event_total_teams: i32,
    pub event_total_participants: i32,
    pub event_female_participants: i32,
    pub event_location_types: Vec<LocationType>,
}
