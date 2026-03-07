use serde::Serialize;

use crate::shared::types::GenderCategory;

#[derive(Debug, Serialize)]
pub struct OrganizerStructure {
    pub id: i32,
    pub name: String,
    pub website_url: String,
    pub competitions: Vec<CompetitionSubStructure>
}

#[derive(Debug, Serialize)]
pub struct CompetitionSubStructure {
    pub id: i32,
    pub name: String,
    pub website_url: String,
    pub gender_category: GenderCategory,
    pub events: Vec<EventSubStructure>
}

#[derive(Debug, Serialize)]
pub struct EventSubStructure {
    pub id: i32,
    pub name: String
}

impl OrganizerStructure {
    pub fn new(id: i32, name: String, website_url: String) -> Self {
        Self { id, name, website_url, competitions: Vec::new() }
    }
}

impl CompetitionSubStructure {
    pub fn new(id: i32, name: String, website_url: String, gender_category: GenderCategory, events: Vec<EventSubStructure>) -> Self {
        Self { id, name, website_url, gender_category, events }
    }
}

impl EventSubStructure {
    pub fn new(id: i32, name: String) -> Self {
        Self { id, name }
    }
}