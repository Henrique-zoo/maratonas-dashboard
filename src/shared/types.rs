use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "gender_category")]
pub enum GenderCategory {
    Open,
    FemaleOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "gender")]
pub enum Gender {
    Male,
    Female,
    Other,
    RatherNotAnswer,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "status")]
pub enum Status {
    Accepted,
    WrongAnswer,
    TimeLimitExceeded,
    MemoryLimitExceeded,
    PresentationError,
    CompilationError,
    RuntimeError,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "role")]
pub enum Role {
    Contestant,
    Coach,
    Reserve,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type, PartialEq, Eq, PartialOrd, Ord)]
#[sqlx(type_name = "location_type")]
pub enum LocationType {
    Continent,
    Country,
    Region,
    Province,
    Prefecture,
    City,
    Campus,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type, PartialEq, Eq, PartialOrd, Ord)]
#[sqlx(type_name = "scope")]
pub enum Scope {
    Global,
    InterContinental,
    Continental,
    International,
    National,
    InterRegional,
    Regional,
    Internal,
}
