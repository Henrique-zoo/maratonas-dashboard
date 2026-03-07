use chrono::NaiveDate;
use serde::Serialize;

use crate::shared::types::GenderCategory;

#[derive(Debug, Serialize)]
pub struct CompetitionStructure {
    pub id: i32,
    pub name: String,
    pub website_url: Option<String>,
    pub gender_category: GenderCategory,
    pub events: Vec<EventSubStructure>
}

#[derive(Debug, Serialize)]
pub struct EventSubStructure {
    pub id: i32,
    pub name: String,
    pub level: Option<i32>,
    pub date: NaiveDate,
    pub location: String,
    pub total_participants: i32,
    pub female_percentage: f32,
    pub teams: Vec<TeamSubStructure>
}

#[derive(Debug, Serialize)]
pub struct TeamSubStructure {
    pub id: i32,
    pub name: String,
    pub total_members: i32,
    pub female_percentage: f32
}

impl CompetitionStructure {
    pub fn new(id: i32, name: String, website_url: Option<String>, gender_category: GenderCategory) -> Self {
        Self { id, name, website_url, gender_category, events: Vec::new() }
    }
}

impl EventSubStructure {
    pub fn new(id: i32, name: String, level: Option<i32>, date: NaiveDate, location: String, total_participants: i32, female_participants: i32, teams: Vec<TeamSubStructure>) -> Self {
        Self {
            id,
            name,
            level,
            date,
            location,
            total_participants,
            female_percentage: female_participants as f32 / total_participants as f32,
            teams
        }
    }
}

impl TeamSubStructure {
    pub fn new(id: i32, name: String, total_members: i32, female_members: i32) -> Self {
        Self {
            id,
            name,
            total_members,
            female_percentage: female_members as f32 / total_members as f32
        }
    }
}