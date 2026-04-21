use async_trait::async_trait;

use crate::{
    errors::AppResult,
    repositories::{
        Registry,
        event::stats,
        types::events::{EventLocationStatsRow, EventYearStatsRow},
    },
    shared::types::LocationType,
};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait EventRepository: Send + Sync {
    async fn find_location_stats(
        &self,
        event_id: i32,
        location_type: LocationType,
        year: i32,
    ) -> AppResult<Vec<EventLocationStatsRow>>;
    async fn find_event_stats_by_year(
        &self,
        event_id: i32,
        year: i32,
    ) -> AppResult<EventYearStatsRow>;
}

#[async_trait]
impl EventRepository for Registry {
    async fn find_location_stats(
        &self,
        event_id: i32,
        location_type: LocationType,
        year: i32,
    ) -> AppResult<Vec<EventLocationStatsRow>> {
        stats::find_location_stats(self, event_id, location_type, year).await
    }

    async fn find_event_stats_by_year(
        &self,
        event_id: i32,
        year: i32,
    ) -> AppResult<EventYearStatsRow> {
        stats::find_event_stats_by_year(self, event_id, year).await
    }
}
