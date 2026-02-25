#[derive(sqlx::FromRow)]
pub struct IdNameRow {
    pub id: i32,
    pub name: String,
}