mod performance_over_time_query;
mod structures_query;

use serde::Deserialize;

use crate::shared::serde::CsvOptVec;

pub use performance_over_time_query::{EventPerformancePath, EventPerformanceQuery};
pub use structures_query::StructuresQuery;

#[derive(Debug, Deserialize)]
pub struct OptionsQuery {
    pub competition_ids: CsvOptVec<i32>,
}
