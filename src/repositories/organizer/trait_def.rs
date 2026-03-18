use async_trait::async_trait;

use crate::{
    errors::AppResult,
    repositories::{
        Registry,
        organizer::{options, structures},
        types::{IdNameRow, organizers::OrganizerStructureRow},
    },
};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait OrganizerRepository: Send + Sync {
    async fn find_options(&self) -> AppResult<Vec<IdNameRow>>;
    async fn find_structures_by_ids(
        &self,
        organizer_ids: Vec<i32>,
    ) -> AppResult<Vec<OrganizerStructureRow>>;
}

#[async_trait]
impl OrganizerRepository for Registry {
    async fn find_options(&self) -> AppResult<Vec<IdNameRow>> {
        options::find_options(self).await
    }

    async fn find_structures_by_ids(
        &self,
        organizer_ids: Vec<i32>,
    ) -> AppResult<Vec<OrganizerStructureRow>> {
        structures::find_structures_by_ids(self, organizer_ids).await
    }
}
