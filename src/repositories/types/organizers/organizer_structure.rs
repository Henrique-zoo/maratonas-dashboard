use chrono::NaiveDate;
use sqlx::FromRow;

use crate::shared::types::{GenderCategory, LocationType};

#[derive(FromRow)]
pub struct OrganizerStructureRow {
    pub organizer_id: i32,
    pub organizer_name: String,
    pub organizer_website_url: Option<String>,

    pub competition_id: i32,
    pub competition_name: String,
    pub competition_website_url: Option<String>,
    pub competition_gender_category: GenderCategory,
    pub competition_years: Vec<i32>,
    pub competition_location_types: Vec<LocationType>,

    pub event_id: i32,
    pub event_name: String,
    pub event_level: Option<i32>,
    pub event_date: NaiveDate,
    pub event_total_institutions: i32,
    pub event_total_teams: i32,
    pub event_total_participants: i32,
    pub event_female_participants: i32,
    pub event_location_types: Vec<LocationType>,
}
