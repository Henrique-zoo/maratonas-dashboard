use indexmap::IndexMap;
use serde::Serialize;

use crate::shared::types::GenderCategory;

// ======================== Response DTOs ========================
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
    pub total_teams: i32,
    pub total_participants: i32,
    pub female_percentage: f32,
    pub events: Vec<EventSubStructure>,
}

#[derive(Debug, Serialize)]
pub struct EventSubStructure {
    pub id: i32,
    pub name: String,
    pub total_teams: i32,
    pub total_participants: i32,
    pub female_percentage: f32,
}

// ======================== Intermediate structures ========================
// Used while aggregating organizer -> competitions -> events
// before converting to the final serializable payload.
#[derive(Debug)]
pub struct TempOrganizerStructure {
    pub id: i32,
    pub name: String,
    pub website_url: String,
    pub competitions: IndexMap<i32, TempCompetitionSubStructure>,
}

#[derive(Debug)]
pub struct TempCompetitionSubStructure {
    pub id: i32,
    pub name: String,
    pub website_url: String,
    pub gender_category: GenderCategory,
    pub total_teams: i32,
    pub total_participants: i32,
    pub female_percentage: f32,
    pub events: IndexMap<i32, EventSubStructure>,
}

// ======================== Conversion to final DTO ========================
impl From<TempOrganizerStructure> for OrganizerStructure {
    fn from(value: TempOrganizerStructure) -> Self {
        Self {
            id: value.id,
            name: value.name,
            website_url: value.website_url,
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
            total_teams: value.total_teams,
            total_participants: value.total_participants,
            female_percentage: value.female_percentage,
            events: value.events.into_values().collect(),
        }
    }
}

// ======================== Helper constructors ========================
impl TempOrganizerStructure {
    pub fn new(
        id: i32,
        name: String,
        website_url: String,
        competitions: IndexMap<i32, TempCompetitionSubStructure>,
    ) -> Self {
        Self {
            id,
            name,
            website_url,
            competitions,
        }
    }
}

impl TempCompetitionSubStructure {
    pub fn new(
        id: i32,
        name: String,
        website_url: String,
        gender_category: GenderCategory,
        total_teams: i32,
        total_participants: i32,
        female_participants: i32,
        events: IndexMap<i32, EventSubStructure>,
    ) -> Self {
        Self {
            id,
            name,
            website_url,
            gender_category,
            total_teams,
            total_participants,
            female_percentage: female_participants as f32 / total_participants as f32,
            events,
        }
    }
}

impl EventSubStructure {
    pub fn new(
        id: i32,
        name: String,
        total_teams: i32,
        total_participants: i32,
        female_participants: i32,
    ) -> Self {
        Self {
            id,
            name,
            total_teams,
            total_participants,
            female_percentage: female_participants as f32 / total_participants as f32,
        }
    }
}
