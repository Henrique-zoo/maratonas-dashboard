use sqlx::FromRow;

use crate::shared::types::GenderCategory;

#[derive(FromRow)]
pub struct OrganizerStructureRow {
    pub organizer_id: i32,
    pub organizer_name: String,
    pub organizer_website_url: String,
    pub competition_id: i32,
    pub competition_name: String,
    pub competition_gender_category: GenderCategory,
    pub competition_website_url: String,
    pub event_id: i32,
    pub event_name: String
}