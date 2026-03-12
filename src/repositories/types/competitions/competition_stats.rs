use sqlx::prelude::FromRow;

#[derive(FromRow)]
pub struct CompetitionLocationStatsRow {
    pub location_id: i32,
    pub location_name: String,
    pub total_institutions: i32,
    pub total_teams: i32,
    pub total_participants: i32,
    pub female_participants: i32,
}
