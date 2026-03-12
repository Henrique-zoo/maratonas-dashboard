use chrono::NaiveDate;
use sqlx::prelude::FromRow;

use crate::shared::types::{GenderCategory, LocationType};

#[derive(FromRow)]
pub struct CompetitionStructureRow {
    pub competition_id: i32,
    pub competition_name: String,
    pub competition_gender_category: GenderCategory,
    pub competition_website_url: Option<String>,

    pub competition_total_teams: i32,
    pub competition_total_participants: i32,
    pub competition_female_participants: i32,

    pub event_id: i32,
    pub event_name: String,
    pub event_level: Option<i32>,
    pub event_date: NaiveDate,
    pub event_location: String,
    pub event_total_teams: i32,
    pub event_total_participants: i32,
    pub event_female_participants: i32,
    pub event_total_institutions: i32,

    pub institution_name: String,
    pub institution_short_name: Option<String>,
    pub institution_location: String,
    pub institution_location_type: LocationType,

    pub team_id: i32,
    pub team_name: String,
    pub team_rank: i32,
    pub team_total_members: i32,
    pub team_female_members: i32,
}
