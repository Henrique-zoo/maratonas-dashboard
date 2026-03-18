mod stats;
mod trait_def;

pub use trait_def::EventRepository;
#[cfg(test)]
pub use trait_def::MockEventRepository;
