use chrono::NaiveDate;
use sqlx::FromRow;

use crate::shared::types::GenderCategory;

#[derive(FromRow)]
pub struct OrganizerStructureRow {
    pub organizer_id: i32,
    pub organizer_name: String,
    pub organizer_website_url: String,

    pub competition_id: i32,
    pub competition_name: String,
    pub competition_website_url: String,
    pub competition_gender_category: GenderCategory,
    pub competition_total_teams: i32,
    pub competition_total_participants: i32,
    pub competition_female_participants: i32,

    pub event_id: i32,
    pub event_name: String,
    pub event_level: i32,
    pub event_date: NaiveDate,
    pub event_total_teams: i32,
    pub event_total_participants: i32,
    pub event_female_participants: i32,
    pub event_years: Vec<i32>,
}
