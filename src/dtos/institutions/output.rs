use indexmap::IndexMap;

use serde::Serialize;

// ======================== Response DTOs ========================
#[derive(Serialize, Debug)]
pub struct InstitutionStructure {
    pub id: i32,
    pub name: String,
    pub total_teams: u32,
    pub total_contestants: u32,
    pub female_percentage: f32,
    pub competitions: Vec<CompetitionSubStructure>,
}

#[derive(Serialize, Debug)]
pub struct CompetitionSubStructure {
    pub id: i32,
    pub name: String,
    pub website_url: Option<String>,
    pub total_teams: u32,
    pub total_participants: u32,
    pub events: Vec<EventSubStructure>,
}

#[derive(Serialize, Debug)]
pub struct EventSubStructure {
    pub id: i32,
    pub name: String,
    pub total_teams: u32,
    pub total_participants: u32,
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
    pub total_teams: u32,
    pub total_contestants: u32,
    pub female_percentage: f32,
    pub competitions: IndexMap<i32, TempCompetitionSubStructure>,
}

#[derive(Debug)]
pub struct TempCompetitionSubStructure {
    pub id: i32,
    pub name: String,
    pub website_url: Option<String>,
    pub total_teams: u32,
    pub total_participants: u32,
    pub events: IndexMap<i32, TempEventSubStructure>,
}

#[derive(Debug)]
pub struct TempEventSubStructure {
    pub id: i32,
    pub name: String,
    pub total_teams: u32,
    pub total_participants: u32,
    pub teams: IndexMap<i32, TeamSubStructure>,
}

// ======================== Conversion to final DTO ========================
impl From<TempInstitutionStructure> for InstitutionStructure {
    fn from(value: TempInstitutionStructure) -> Self {
        Self {
            id: value.id,
            name: value.name,
            total_teams: value.total_teams,
            total_contestants: value.total_contestants,
            female_percentage: value.female_percentage,
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
            total_teams: value.total_teams,
            total_participants: value.total_participants,
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
            total_teams: value.total_teams,
            total_participants: value.total_participants,
            teams: { value.teams.into_values().collect() },
        }
    }
}

// ======================== Helper constructors ========================
impl TempInstitutionStructure {
    pub fn new(
        id: i32,
        name: String,
        total_teams: i32,
        total_contestants: i32,
        female_contestants: i32,
        competitions: IndexMap<i32, TempCompetitionSubStructure>,
    ) -> Self {
        Self {
            id,
            name,
            total_teams: total_teams as u32,
            total_contestants: total_contestants as u32,
            female_percentage: female_contestants as f32 / total_contestants as f32,
            competitions,
        }
    }
}

impl TempCompetitionSubStructure {
    pub fn new(
        id: i32,
        name: String,
        website_url: Option<String>,
        total_teams: i32,
        total_participants: i32,
        events: IndexMap<i32, TempEventSubStructure>,
    ) -> Self {
        Self {
            id,
            name,
            website_url,
            total_teams: total_teams as u32,
            total_participants: total_participants as u32,
            events,
        }
    }
}

impl TempEventSubStructure {
    pub fn new(
        id: i32,
        name: String,
        total_teams: i32,
        total_participants: i32,
        teams: IndexMap<i32, TeamSubStructure>,
    ) -> Self {
        Self {
            id,
            name,
            total_teams: total_teams as u32,
            total_participants: total_participants as u32,
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
