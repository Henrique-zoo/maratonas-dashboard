use chrono::NaiveDate;
use indexmap::IndexMap;
use serde::Serialize;

use crate::shared::types::{GenderCategory, Scope};

// ======================== Response DTOs ========================
#[derive(Debug, Serialize)]
pub struct TeamStructure {
    pub id: i32,
    pub name: String,
    pub competitions: Vec<CompetitionSubStructure>,
}

#[derive(Debug, Serialize)]
pub struct CompetitionSubStructure {
    pub id: i32,
    pub name: String,
    pub website_url: Option<String>,
    pub gender_category: GenderCategory,
    pub years: Vec<u32>,
    pub total_members: u32,
    pub female_percentage: f32,
    pub events: Vec<EventSubStructure>,
}

#[derive(Debug, Serialize)]
pub struct EventSubStructure {
    pub id: i32,
    pub name: String,
    pub level: Option<u32>,
    pub date: NaiveDate,
    pub location: String,
    pub scope: Scope,
    pub team_event_rank: u32,
}

// ======================== Intermediate structures ========================
// Used while aggregating teams -> competitions -> events
// before converting to the final serializable payload.
#[derive(Debug)]
pub struct TempTeamStructure {
    pub id: i32,
    pub name: String,
    pub competitions: IndexMap<i32, TempCompetitionSubStructure>,
}

#[derive(Debug)]
pub struct TempCompetitionSubStructure {
    pub id: i32,
    pub name: String,
    pub website_url: Option<String>,
    pub gender_category: GenderCategory,
    pub years: Vec<u32>,
    pub total_members: u32,
    pub female_percentage: f32,
    pub events: IndexMap<i32, EventSubStructure>,
}

impl From<TempTeamStructure> for TeamStructure {
    fn from(value: TempTeamStructure) -> Self {
        Self {
            id: value.id,
            name: value.name,
            competitions: value
                .competitions
                .into_values()
                .map(CompetitionSubStructure::from)
                .collect(),
        }
    }
}

impl From<TempCompetitionSubStructure> for CompetitionSubStructure {
    fn from(value: TempCompetitionSubStructure) -> Self {
        Self {
            id: value.id,
            name: value.name,
            website_url: value.website_url,
            gender_category: value.gender_category,
            years: value.years,
            total_members: value.total_members,
            female_percentage: value.female_percentage,
            events: value.events.into_values().collect(),
        }
    }
}

impl TempTeamStructure {
    pub fn new(
        id: i32,
        name: String,
        competitions: IndexMap<i32, TempCompetitionSubStructure>,
    ) -> Self {
        Self {
            id,
            name,
            competitions,
        }
    }
}

impl TempCompetitionSubStructure {
    pub fn new(
        id: i32,
        name: String,
        website_url: Option<String>,
        gender_category: GenderCategory,
        years: Vec<i32>,
        total_members: i32,
        female_members: i32,
        events: IndexMap<i32, EventSubStructure>,
    ) -> Self {
        Self {
            id,
            name,
            website_url,
            gender_category,
            years: years.into_iter().map(|y| y as u32).collect(),
            total_members: total_members as u32,
            female_percentage: female_members as f32 / total_members as f32,
            events,
        }
    }
}

impl EventSubStructure {
    pub fn new(
        id: i32,
        name: String,
        level: Option<i32>,
        date: NaiveDate,
        location: String,
        scope: Scope,
        team_event_rank: i32,
    ) -> Self {
        Self {
            id,
            name,
            level: level.map(|l| l as u32),
            date,
            location,
            scope,
            team_event_rank: team_event_rank as u32,
        }
    }
}
