use async_trait::async_trait;

use crate::{
    errors::AppResult,
    repositories::{
        Registry,
        competition::{options, stats, structures},
        types::{
            IdNameRow,
            competitions::{
                CompetitionEventsByYearRow, CompetitionLocationStatsRow, CompetitionStructureRow,
                CompetitionTeamYearResultRow, CompetitionYearStatsRow, CompetitionYearStructureRow,
            },
        },
    },
    shared::types::LocationType,
};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait CompetitionRepository: Send + Sync {
    async fn find_options_by_organizers(
        &self,
        organizer_ids: Option<Vec<i32>>,
    ) -> AppResult<Vec<IdNameRow>>;
    async fn find_structures_by_ids(
        &self,
        competition_ids: Vec<i32>,
    ) -> AppResult<Vec<CompetitionStructureRow>>;
    async fn find_location_stats_by_competition(
        &self,
        competition_id: i32,
        location_type: LocationType,
        year: i32,
    ) -> AppResult<Vec<CompetitionLocationStatsRow>>;
    async fn find_events_by_year(
        &self,
        competition_id: i32,
        year: i32,
    ) -> AppResult<Vec<CompetitionEventsByYearRow>>;
    async fn find_structure_by_year(
        &self,
        competition_id: i32,
        year: i32,
    ) -> AppResult<Vec<CompetitionYearStructureRow>>;
    async fn find_competition_stats_by_year(
        &self,
        competition_id: i32,
        year: i32,
    ) -> AppResult<CompetitionYearStatsRow>;
    async fn find_team_result_by_year(
        &self,
        team_id: i32,
        competition_id: i32,
        year: i32,
    ) -> AppResult<Vec<CompetitionTeamYearResultRow>>;
}

#[async_trait]
impl CompetitionRepository for Registry {
    async fn find_options_by_organizers(
        &self,
        organizer_ids: Option<Vec<i32>>,
    ) -> AppResult<Vec<IdNameRow>> {
        options::find_options_by_organizers(self, organizer_ids).await
    }

    async fn find_structures_by_ids(
        &self,
        competition_ids: Vec<i32>,
    ) -> AppResult<Vec<CompetitionStructureRow>> {
        structures::find_structures_by_ids(self, competition_ids).await
    }

    async fn find_location_stats_by_competition(
        &self,
        competition_id: i32,
        location_type: LocationType,
        year: i32,
    ) -> AppResult<Vec<CompetitionLocationStatsRow>> {
        stats::find_location_stats_by_competition(self, competition_id, location_type, year).await
    }

    async fn find_events_by_year(
        &self,
        competition_id: i32,
        year: i32,
    ) -> AppResult<Vec<CompetitionEventsByYearRow>> {
        structures::find_events_by_year(self, competition_id, year).await
    }

    async fn find_structure_by_year(
        &self,
        competition_id: i32,
        year: i32,
    ) -> AppResult<Vec<CompetitionYearStructureRow>> {
        structures::find_structure_by_year(self, competition_id, year).await
    }

    async fn find_competition_stats_by_year(
        &self,
        competition_id: i32,
        year: i32,
    ) -> AppResult<CompetitionYearStatsRow> {
        stats::find_competition_stats_by_year(self, competition_id, year).await
    }

    async fn find_team_result_by_year(
        &self,
        team_id: i32,
        competition_id: i32,
        year: i32,
    ) -> AppResult<Vec<CompetitionTeamYearResultRow>> {
        structures::find_team_result_by_year(self, team_id, competition_id, year).await
    }
}
