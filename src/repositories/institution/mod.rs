mod options;
mod performance;
mod structures;
mod trait_def;

pub use trait_def::InstitutionRepository;
#[cfg(test)]
pub use trait_def::MockInstitutionRepository;
