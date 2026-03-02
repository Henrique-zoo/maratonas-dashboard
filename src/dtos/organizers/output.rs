use serde::Serialize;

use crate::shared::GenderCategory;

#[derive(Debug, Serialize)]
pub struct OrganizerStructure {
    pub id: i32,
    pub name: String,
    pub website_url: String,
    pub competitions: Vec<CompetitionSubStructure>,
}

#[derive(Debug, Serialize)]
pub struct CompetitionSubStructure {
    pub id: i32,
    pub name: String,
    pub website_url: String,
    pub gender_category: GenderCategory,
    pub events: Vec<EventSubStructure>,
}

#[derive(Debug, Serialize)]
pub struct EventSubStructure {
    pub id: i32,
    pub name: String,
}