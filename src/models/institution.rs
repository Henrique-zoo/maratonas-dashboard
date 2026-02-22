use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Institution {
    pub id: i32,
    pub name: String,
    pub short_name: Option<String>,
    pub site: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct InstitutionLocation {
    pub institution_id: i32,
    pub location_id: i32,
}