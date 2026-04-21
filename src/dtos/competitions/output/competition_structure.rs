use chrono::NaiveDate;
use indexmap::IndexMap;
use serde::Serialize;

use crate::shared::types::{GenderCategory, LocationType};

// ======================== Response DTOs ========================
#[derive(Debug, Serialize)]
pub struct CompetitionStructure {
    pub id: i32,
    pub name: String,
    pub website_url: Option<String>,
    pub gender_category: GenderCategory,
    pub years: Vec<u32>,
    pub location_types: Vec<LocationType>,
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

// Used for updating competition by year in organizer page
#[derive(Default, Debug, Serialize)]
pub struct OrganizerCompetitionYearStructure {
    pub location_types: Vec<LocationType>,
    pub events: Vec<crate::dtos::organizers::output::EventSubStructure>,
}

// Used for updating competition by year in competition page
#[derive(Debug, Serialize)]
pub struct CompetitionYearStructure {
    pub location_types: Vec<LocationType>,
    pub events: Vec<EventSubStructure>,
}

// Used for updating competition by year in competition page
#[derive(Default, Debug, Serialize)]
pub struct TeamCompetitionYearStructure {
    pub events: Vec<crate::dtos::teams::output::EventSubStructure>,
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
    pub years: Vec<u32>,
    pub location_types: Vec<LocationType>,
    pub events: IndexMap<i32, TempEventSubStructure>,
}

#[derive(Debug)]
pub struct TempEventSubStructure {
    pub id: i32,
    pub name: String,
    pub level: Option<u32>,
    pub date: NaiveDate,
    pub location: String,
    pub location_types: Vec<LocationType>,
    pub teams: IndexMap<i32, TeamSubStructure>,
}

#[derive(Default, Debug)]
pub struct TempCompetitionYearStructure {
    pub location_types: Vec<LocationType>,
    pub events: IndexMap<i32, TempEventSubStructure>,
}

// ======================== Conversion to final DTO ========================
impl From<TempCompetitionStructure> for CompetitionStructure {
    fn from(value: TempCompetitionStructure) -> Self {
        let mut location_types = value.location_types;
        location_types.sort();
        Self {
            id: value.id,
            name: value.name,
            website_url: value.website_url,
            gender_category: value.gender_category,
            years: value.years,
            location_types,
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
        let mut location_types = value.location_types;
        location_types.sort();
        Self {
            id: value.id,
            name: value.name,
            level: value.level,
            date: value.date,
            location: value.location,
            location_types,
            teams: value.teams.into_values().collect(),
        }
    }
}

impl From<TempCompetitionYearStructure> for CompetitionYearStructure {
    fn from(value: TempCompetitionYearStructure) -> Self {
        let mut location_types = value.location_types;
        location_types.sort();
        Self {
            location_types,
            events: value
                .events
                .into_values()
                .map(EventSubStructure::from)
                .collect(),
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
        years: Vec<i32>,
        location_types: Vec<LocationType>,
        events: IndexMap<i32, TempEventSubStructure>,
    ) -> Self {
        Self {
            id,
            name,
            website_url,
            gender_category,
            years: years.into_iter().map(|y| y as u32).collect(),
            location_types,
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
        location_types: Vec<LocationType>,
        teams: IndexMap<i32, TeamSubStructure>,
    ) -> Self {
        Self {
            id,
            name,
            level: level.map(|l| l as u32),
            date,
            location,
            location_types,
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

impl OrganizerCompetitionYearStructure {
    pub fn update(&mut self, location_types: Vec<LocationType>) {
        self.location_types = location_types;
    }
}

impl TempCompetitionYearStructure {
    pub fn update(&mut self, location_types: Vec<LocationType>) {
        self.location_types = location_types;
    }
}
