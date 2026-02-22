use serde::{Serialize, Deserialize};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Author {
    pub id: i32,
    pub name: String,
    pub nationality: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Authorship {
    pub author_id: i32,
    pub problem_id: i32,
}