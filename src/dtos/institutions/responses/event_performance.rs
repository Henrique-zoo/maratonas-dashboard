use serde::Serialize;

use crate::repositories::types::institutions::EventPerformanceRow;

#[derive(Debug, Serialize)]
pub struct EventPerformance {
    pub year: i32,
    pub best_performance_rank: i32,
    pub best_performance_team_id: i32,
    pub best_performance_team_name: String,
    pub medium_performance_rank: f32,
}

impl From<EventPerformanceRow> for EventPerformance {
    fn from(value: EventPerformanceRow) -> Self {
        Self {
            year: value.year,
            best_performance_rank: value.best_performance_rank,
            best_performance_team_id: value.best_performance_team_id,
            best_performance_team_name: value.best_performance_team_name,
            medium_performance_rank: value.medium_performance_rank,
        }
    }
}
