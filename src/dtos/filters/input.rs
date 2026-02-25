use serde::Deserialize;

#[derive(Deserialize)]
pub struct CompetitionOptionsQuery {
    pub organizer_ids: Option<Vec<i32>>,
}