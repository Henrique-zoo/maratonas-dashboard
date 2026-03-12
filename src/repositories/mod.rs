// ============= SUB MÓDULOS =============
mod competition_repository;
mod event_repository;
mod institution_repository;
mod organizer_repository;
mod registry;
mod team_repository;
pub(crate) mod types;

/*
*************************************************
***********   ********        *******    ********
**********     *******   ***   ******    ********
*********  ***  ******   ****   *****    ********
********  *****  *****   ***   ******    ********
*******           ****       ********    ********
******   *******   ***    ***********    ********
*****   *********   **    ***********    ********  de repositories
*************************************************
*/
// ============= STRUCTS =============
pub use registry::Registry;
// ============= TRAITS =============
pub use competition_repository::CompetitionRepository;
pub use event_repository::EventRepository;
pub use institution_repository::InstitutionRepository;
pub use organizer_repository::OrganizerRepository;
pub use team_repository::TeamRepository;

// ============= MOCKS (only available in tests) =============
#[cfg(test)]
pub use competition_repository::MockCompetitionRepository;
#[cfg(test)]
pub use event_repository::MockEventRepository;
#[cfg(test)]
pub use institution_repository::MockInstitutionRepository;
#[cfg(test)]
pub use organizer_repository::MockOrganizerRepository;
#[cfg(test)]
pub use team_repository::MockTeamRepository;
