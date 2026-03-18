use sqlx::prelude::FromRow;

#[derive(FromRow)]
pub struct EventPerformanceRow {
    pub year: i32,
    pub best_performance_rank: i32,
    pub best_performance_team_id: i32,
    pub best_performance_team_name: String,
    pub medium_performance_rank: f32,
}
