use sqlx::FromRow;

#[derive(FromRow)]
pub struct TeamStructureRow {
    pub team_id: i32,
    pub team_name: String,
    pub competition_id: i32,
    pub competition_name: String,
    pub event_id: i32,
    pub event_name: String,
    pub event_total_participants: i32,
}
