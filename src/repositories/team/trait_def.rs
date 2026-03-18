use async_trait::async_trait;

use crate::{
    errors::AppResult,
    repositories::{
        Registry,
        team::{options, structures},
        types::{IdNameRow, teams::TeamStructureRow},
    },
};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait TeamRepository: Send + Sync {
    async fn find_options_by_competitions_and_instructions(
        &self,
        competition_ids: Option<Vec<i32>>,
        institution_ids: Option<Vec<i32>>,
    ) -> AppResult<Vec<IdNameRow>>;
    async fn find_structures_by_ids(&self, team_ids: Vec<i32>) -> AppResult<Vec<TeamStructureRow>>;
}

#[async_trait]
impl TeamRepository for Registry {
    async fn find_options_by_competitions_and_instructions(
        &self,
        competition_ids: Option<Vec<i32>>,
        institution_ids: Option<Vec<i32>>,
    ) -> AppResult<Vec<IdNameRow>> {
        options::find_options_by_competitions_and_instructions(
            self,
            competition_ids,
            institution_ids,
        )
        .await
    }

    async fn find_structures_by_ids(&self, team_ids: Vec<i32>) -> AppResult<Vec<TeamStructureRow>> {
        structures::find_structures_by_ids(self, team_ids).await
    }
}
