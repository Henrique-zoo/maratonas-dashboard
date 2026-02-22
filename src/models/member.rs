use serde::{Serialize, Deserialize};
use crate::shared::{Gender, Role};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Member {
    pub id: i32,
    pub gender: Gender,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct TeamEventMember {
    pub member_id: i32,
    pub team_event_id: i32,
    pub role: Role,
}