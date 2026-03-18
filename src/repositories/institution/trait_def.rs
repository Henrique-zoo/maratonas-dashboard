use async_trait::async_trait;

use crate::{
    errors::AppResult,
    repositories::{
        Registry,
        institution::{options, performance, structures},
        types::{
            IdNameRow,
            institutions::{EventPerformanceRow, InstitutionStructureRow},
        },
    },
};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait InstitutionRepository: Send + Sync {
    async fn find_options_by_competitions(
        &self,
        competition_ids: Option<Vec<i32>>,
    ) -> AppResult<Vec<IdNameRow>>;
    async fn find_structures_by_ids(
        &self,
        institution_ids: Vec<i32>,
    ) -> AppResult<Vec<InstitutionStructureRow>>;
    async fn find_event_performance_over_time(
        &self,
        institution_id: i32,
        event_id: i32,
        star_year: i32,
        end_year: i32,
    ) -> AppResult<Vec<EventPerformanceRow>>;
}

#[async_trait]
impl InstitutionRepository for Registry {
    async fn find_options_by_competitions(
        &self,
        competition_ids: Option<Vec<i32>>,
    ) -> AppResult<Vec<IdNameRow>> {
        options::find_options_by_competitions(self, competition_ids).await
    }

    async fn find_structures_by_ids(
        &self,
        institution_ids: Vec<i32>,
    ) -> AppResult<Vec<InstitutionStructureRow>> {
        structures::find_structures_by_ids(self, institution_ids).await
    }

    async fn find_event_performance_over_time(
        &self,
        institution_id: i32,
        event_id: i32,
        star_year: i32,
        end_year: i32,
    ) -> AppResult<Vec<EventPerformanceRow>> {
        performance::find_event_performance_over_time(
            self,
            institution_id,
            event_id,
            star_year,
            end_year,
        )
        .await
    }
}
