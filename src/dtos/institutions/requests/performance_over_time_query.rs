use serde::Deserialize;

#[derive(Deserialize)]
pub struct EventPerformancePath {
    pub institution_id: i32,
    pub event_id: i32,
}

#[derive(Deserialize)]
pub struct EventPerformanceQuery {
    pub start_year: Option<i32>,
    pub end_year: Option<i32>,
}
