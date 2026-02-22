use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Problem {
    pub id: i32,
    pub event_id: i32,
    pub item: String,
    pub title: String,
    pub statement: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct InputOutput {
    pub id: i32,
    pub problem_id: i32,
    pub input: String,
    pub output: String,
}