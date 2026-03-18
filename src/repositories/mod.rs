// ============= SUB MÓDULOS =============
mod competition;
mod event;
mod institution;
mod organizer;
mod registry;
mod team;
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
pub use competition::CompetitionRepository;
pub use event::EventRepository;
pub use institution::InstitutionRepository;
pub use organizer::OrganizerRepository;
pub use team::TeamRepository;

// ============= MOCKS (only available in tests) =============
#[cfg(test)]
pub use competition::MockCompetitionRepository;
#[cfg(test)]
pub use event::MockEventRepository;
#[cfg(test)]
pub use institution::MockInstitutionRepository;
#[cfg(test)]
pub use organizer::MockOrganizerRepository;
#[cfg(test)]
pub use team::MockTeamRepository;
