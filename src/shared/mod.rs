use serde::{Serialize, Deserialize};
use sqlx::Type;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "gendercategory")]
pub enum GenderCategory {
    Open,
    FemaleOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "gender")]
pub enum Gender {
    Male,
    Female,
    Other,
    RatherNotAnswer,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "role")]
pub enum Role {
    Contestant,
    Coach,
    Reserve,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "locationtype")]
pub enum LocationType {
    Continent,
    Country,
    Region,
    Province,
    Prefecture,
    City,
    Campus,
}