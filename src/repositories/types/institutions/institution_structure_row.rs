use chrono::NaiveDate;
use sqlx::prelude::FromRow;

#[derive(FromRow)]
pub struct InstitutionStructureRow {
    pub institution_id: i32,
    pub institution_name: String,
    pub institution_total_teams: i32,
    pub institution_total_contestants: i32,
    pub institution_female_contestants: i32,

    pub competition_id: i32,
    pub competition_name: String,
    pub competition_website_url: Option<String>,
    pub competition_total_teams: i32,
    pub competition_total_participants: i32,

    pub event_id: i32,
    pub event_name: String,
    pub event_date: NaiveDate,
    pub event_total_teams: i32,
    pub event_total_participants: i32,

    pub team_id: i32,
    pub team_name: String,
    pub team_event_rank: i32,
    pub team_total_members: i32,
    pub team_female_members: i32,
}
