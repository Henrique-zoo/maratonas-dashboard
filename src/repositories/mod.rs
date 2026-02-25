// ============= SUB MÓDULOS =============
mod registry;
mod organizer_repository;
mod competition_repository;
mod institution_repository;
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
pub use organizer_repository::OrganizerRepository;
pub use competition_repository::CompetitionRepository;
pub use institution_repository::InstitutionRepository;
pub use team_repository::TeamRepository;