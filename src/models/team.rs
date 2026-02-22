use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub institution_id: i32,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct TeamEvent {
    pub id: i32,
    pub team_id: i32,
    pub event_id: i32,
    pub rank: i32,
}