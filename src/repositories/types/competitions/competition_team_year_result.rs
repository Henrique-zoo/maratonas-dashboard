use chrono::NaiveDate;
use sqlx::prelude::FromRow;

use crate::shared::types::Scope;

#[derive(FromRow)]
pub struct CompetitionTeamYearResultRow {
    pub team_total_members: i32,
    pub team_female_members: i32,

    pub event_id: i32,
    pub event_name: String,
    pub event_level: Option<i32>,
    pub event_date: NaiveDate,
    pub event_location: String,
    pub event_scope: Scope,
    pub team_event_rank: i32,
}
