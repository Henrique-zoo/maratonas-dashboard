use chrono::NaiveDate;
use sqlx::prelude::FromRow;

use crate::shared::types::GenderCategory;

#[derive(FromRow)]
pub struct CompetitionStructureRow {
    pub competition_id: i32,
    pub competition_name: String,
    pub competition_gender_category: GenderCategory,
    pub competition_website_url: Option<String>,
    pub event_id: i32,
    pub event_name: String,
    pub event_level: Option<i32>,
    pub event_date: NaiveDate,
    pub event_location: String,
    pub event_total_participants: i32,
    pub event_female_participants: i32,
    pub team_id: i32,
    pub team_name: String,
    pub team_total_members: i32,
    pub team_female_members: i32
}