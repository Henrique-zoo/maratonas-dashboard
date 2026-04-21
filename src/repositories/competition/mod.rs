mod options;
mod stats;
mod structures;
mod trait_def;

pub use trait_def::CompetitionRepository;

#[cfg(test)]
pub use trait_def::MockCompetitionRepository;
