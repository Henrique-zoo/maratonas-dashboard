use chrono::NaiveDate;
use indexmap::IndexMap;
use serde::Serialize;
use std::collections::BTreeSet;

use crate::shared::types::{GenderCategory, LocationType};

// ======================== Response DTOs ========================
#[derive(Debug, Serialize)]
pub struct CompetitionStructure {
    pub id: i32,
    pub name: String,
    pub website_url: Option<String>,
    pub gender_category: GenderCategory,
    pub location_types: Vec<LocationType>,
    pub total_teams: u32,
    pub total_participants: u32,
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
    pub location_types: Vec<LocationType>,
    pub total_teams: u32,
    pub total_participants: u32,
    pub female_percentage: f32,
    pub teams: Vec<TeamSubStructure>,
}

#[derive(Debug, Serialize)]
pub struct TeamSubStructure {
    pub id: i32,
    pub name: String,
    pub rank: u32,
    pub institution_name: String,
    pub institution_short_name: Option<String>,
    pub institution_location: String,
    pub total_members: u32,
    pub female_percentage: f32,
}

// ======================== Intermediate structures ========================
// Used while aggregating data (competition -> events -> teams)
// before converting to the final serializable payload.
#[derive(Debug)]
pub struct TempCompetitionStructure {
    pub id: i32,
    pub name: String,
    pub website_url: Option<String>,
    pub gender_category: GenderCategory,
    pub location_types: BTreeSet<LocationType>,
    pub total_teams: u32,
    pub total_participants: u32,
    pub female_percentage: f32,
    pub events: IndexMap<i32, TempEventSubStructure>,
}

#[derive(Debug)]
pub struct TempEventSubStructure {
    pub id: i32,
    pub name: String,
    pub level: Option<u32>,
    pub date: NaiveDate,
    pub location: String,
    pub location_types: BTreeSet<LocationType>,
    pub total_teams: u32,
    pub total_participants: u32,
    pub female_percentage: f32,
    pub teams: IndexMap<i32, TeamSubStructure>,
}

// ======================== Conversion to final DTO ========================
impl From<TempCompetitionStructure> for CompetitionStructure {
    fn from(value: TempCompetitionStructure) -> Self {
        Self {
            id: value.id,
            name: value.name,
            website_url: value.website_url,
            gender_category: value.gender_category,
            location_types: value.location_types.into_iter().collect(),
            total_teams: value.total_teams,
            total_participants: value.total_participants,
            female_percentage: value.female_percentage,
            events: value
                .events
                .into_values()
                .map(EventSubStructure::from)
                .collect(),
        }
    }
}

impl From<TempEventSubStructure> for EventSubStructure {
    fn from(value: TempEventSubStructure) -> Self {
        Self {
            id: value.id,
            name: value.name,
            level: value.level,
            date: value.date,
            location: value.location,
            location_types: value.location_types.into_iter().collect(),
            total_teams: value.total_teams,
            total_participants: value.total_participants,
            female_percentage: value.female_percentage,
            teams: value.teams.into_values().collect(),
        }
    }
}

// ======================== Helper constructors ========================
impl TempCompetitionStructure {
    pub fn new(
        id: i32,
        name: String,
        website_url: Option<String>,
        gender_category: GenderCategory,
        location_types: BTreeSet<LocationType>,
        total_teams: i32,
        total_participants: i32,
        female_participants: i32,
        events: IndexMap<i32, TempEventSubStructure>,
    ) -> Self {
        Self {
            id,
            name,
            website_url,
            gender_category,
            location_types,
            total_teams: total_teams as u32,
            total_participants: total_participants as u32,
            female_percentage: female_participants as f32 / total_participants as f32,
            events,
        }
    }
}

impl TempEventSubStructure {
    pub fn new(
        id: i32,
        name: String,
        level: Option<i32>,
        date: NaiveDate,
        location: String,
        location_types: BTreeSet<LocationType>,
        total_teams: i32,
        total_participants: i32,
        female_participants: i32,
        teams: IndexMap<i32, TeamSubStructure>,
    ) -> Self {
        Self {
            id,
            name,
            level: level.map(|l| l as u32),
            date,
            location,
            location_types,
            total_teams: total_teams as u32,
            total_participants: total_participants as u32,
            female_percentage: female_participants as f32 / total_participants as f32,
            teams,
        }
    }
}

impl TeamSubStructure {
    pub fn new(
        id: i32,
        name: String,
        rank: i32,
        institution_name: String,
        institution_short_name: Option<String>,
        institution_location: String,
        total_members: i32,
        female_members: i32,
    ) -> Self {
        Self {
            id,
            name,
            rank: rank as u32,
            institution_name,
            institution_short_name,
            institution_location,
            total_members: total_members as u32,
            female_percentage: female_members as f32 / total_members as f32,
        }
    }
}
