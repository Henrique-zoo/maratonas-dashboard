use chrono::NaiveDate;
use indexmap::IndexMap;

use serde::Serialize;

use crate::shared::types::Scope;

// ======================== Response DTOs ========================
#[derive(Serialize, Debug)]
pub struct InstitutionStructure {
    pub id: i32,
    pub name: String,
    pub short_name: Option<String>,
    pub location: String,
    pub competitions: Vec<CompetitionSubStructure>,
}

#[derive(Serialize, Debug)]
pub struct CompetitionSubStructure {
    pub id: i32,
    pub name: String,
    pub website_url: Option<String>,
    pub events: Vec<EventSubStructure>,
}

#[derive(Serialize, Debug)]
pub struct EventSubStructure {
    pub id: i32,
    pub name: String,
    pub date: NaiveDate,
    pub level: Option<u32>,
    pub scope: Scope,
    pub teams: Vec<TeamSubStructure>,
}

#[derive(Serialize, Debug)]
pub struct TeamSubStructure {
    pub id: i32,
    pub name: String,
    pub rank: u32,
    pub total_members: u32,
    pub female_percentage: f32,
}

// ======================== Intermediate structures ========================
// Used while aggregating institution -> competitions -> events -> teams
// before converting to the final serializable payload.
#[derive(Debug)]
pub struct TempInstitutionStructure {
    pub id: i32,
    pub name: String,
    pub short_name: Option<String>,
    pub location: String,
    pub competitions: IndexMap<i32, TempCompetitionSubStructure>,
}

#[derive(Debug)]
pub struct TempCompetitionSubStructure {
    pub id: i32,
    pub name: String,
    pub website_url: Option<String>,
    pub events: IndexMap<i32, TempEventSubStructure>,
}

#[derive(Debug)]
pub struct TempEventSubStructure {
    pub id: i32,
    pub name: String,
    pub date: NaiveDate,
    pub level: Option<u32>,
    pub scope: Scope,
    pub teams: IndexMap<i32, TeamSubStructure>,
}

// ======================== Conversion to final DTO ========================
impl From<TempInstitutionStructure> for InstitutionStructure {
    fn from(value: TempInstitutionStructure) -> Self {
        Self {
            id: value.id,
            name: value.name,
            short_name: value.short_name,
            location: value.location,
            competitions: {
                value
                    .competitions
                    .into_values()
                    .map(CompetitionSubStructure::from)
                    .collect()
            },
        }
    }
}

impl From<TempCompetitionSubStructure> for CompetitionSubStructure {
    fn from(value: TempCompetitionSubStructure) -> Self {
        Self {
            id: value.id,
            name: value.name,
            website_url: value.website_url,
            events: {
                value
                    .events
                    .into_values()
                    .map(EventSubStructure::from)
                    .collect()
            },
        }
    }
}

impl From<TempEventSubStructure> for EventSubStructure {
    fn from(value: TempEventSubStructure) -> Self {
        Self {
            id: value.id,
            name: value.name,
            date: value.date,
            level: value.level,
            scope: value.scope,
            teams: { value.teams.into_values().collect() },
        }
    }
}

// ======================== Helper constructors ========================
impl TempInstitutionStructure {
    pub fn new(
        id: i32,
        name: String,
        short_name: Option<String>,
        location: String,
        competitions: IndexMap<i32, TempCompetitionSubStructure>,
    ) -> Self {
        Self {
            id,
            name,
            short_name,
            location,
            competitions,
        }
    }
}

impl TempCompetitionSubStructure {
    pub fn new(
        id: i32,
        name: String,
        website_url: Option<String>,
        events: IndexMap<i32, TempEventSubStructure>,
    ) -> Self {
        Self {
            id,
            name,
            website_url,
            events,
        }
    }
}

impl TempEventSubStructure {
    pub fn new(
        id: i32,
        name: String,
        date: NaiveDate,
        level: Option<i32>,
        scope: Scope,
        teams: IndexMap<i32, TeamSubStructure>,
    ) -> Self {
        Self {
            id,
            name,
            date,
            level: level.map(|l| l as u32),
            scope,
            teams,
        }
    }
}

impl TeamSubStructure {
    pub fn new(id: i32, name: String, rank: i32, total_members: i32, female_members: i32) -> Self {
        Self {
            id,
            name,
            rank: rank as u32,
            total_members: total_members as u32,
            female_percentage: female_members as f32 / total_members as f32,
        }
    }
}
