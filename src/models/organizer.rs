use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Organizer {
    pub id: i32,
    pub name: String,
    pub website_url: Option<String>,
}