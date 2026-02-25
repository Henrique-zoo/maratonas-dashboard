use serde::Deserialize;

#[derive(Deserialize)]
pub struct CompetitionOptionsQuery {
    pub organizer_ids: Option<Vec<i32>>
}

#[derive(Deserialize)]
pub struct InstitutionOptionsQuery {
    pub competition_ids: Option<Vec<i32>>
}

#[derive(Deserialize)]
pub struct TeamOptionQuery {
    pub competition_ids: Option<Vec<i32>>,
    pub institution_ids: Option<Vec<i32>>
}