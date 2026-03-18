use chrono::NaiveDate;
use sqlx::prelude::FromRow;

use crate::shared::types::Scope;

#[derive(FromRow)]
pub struct InstitutionStructureRow {
    pub institution_id: i32,
    pub institution_name: String,
    pub institution_short_name: Option<String>,
    pub institution_location: String,

    pub competition_id: i32,
    pub competition_name: String,
    pub competition_website_url: Option<String>,

    pub event_id: i32,
    pub event_name: String,
    pub event_date: NaiveDate,
    pub event_level: Option<i32>,
    pub event_scope: Scope,

    pub team_id: i32,
    pub team_name: String,
    pub team_event_rank: i32,
    pub team_total_members: i32,
    pub team_female_members: i32,
}
