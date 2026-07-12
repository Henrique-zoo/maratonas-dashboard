//! # `backend::shared::types`
//!
//! ## Responsabilidade
//! Define enums de domínio compartilhados entre SQL, serviços e DTOs.
//!
//! ## Lógica de Implementação
//! Mantém categorias canônicas serializáveis/deserializáveis para evitar divergência semântica entre camadas.
//!
//! ## Funções
//! Este arquivo não declara funções de produção neste escopo.
//!
//! ## Tipos
//! - `GenderCategory`: Enum de domínio compartilhado entre persistência, serviços e camada HTTP.
//! - `Gender`: Enum de domínio compartilhado entre persistência, serviços e camada HTTP.
//! - `Status`: Enum de domínio compartilhado entre persistência, serviços e camada HTTP.
//! - `Role`: Enum de domínio compartilhado entre persistência, serviços e camada HTTP.
//! - `LocationType`: Enum de domínio compartilhado entre persistência, serviços e camada HTTP.
//! - `Scope`: Enum de domínio compartilhado entre persistência, serviços e camada HTTP.
//!
use serde::{Deserialize, Serialize};
use sqlx::Type;

/// Categoria de gênero associada a competições.
///
/// Espelha o tipo PostgreSQL `gender_category` e também é serializada nos DTOs
/// que descrevem competições.
#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "gender_category")]
pub enum GenderCategory {
    /// Competição aberta.
    Open,
    /// Competição restrita à categoria feminina.
    FemaleOnly,
}

/// Gênero declarado por membros.
///
/// Espelha o tipo PostgreSQL `gender` e é usado principalmente em agregações
/// de participantes femininas.
#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "gender")]
pub enum Gender {
    /// Gênero masculino.
    Male,
    /// Gênero feminino.
    Female,
    /// Outro gênero declarado.
    Other,
    /// Preferência por não informar o gênero.
    RatherNotAnswer,
}

/// Status de submissão ou julgamento.
///
/// Espelha o tipo PostgreSQL `status` e representa resultados canônicos de
/// avaliação.
#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "status")]
pub enum Status {
    /// Submissão aceita.
    Accepted,
    /// Resposta incorreta.
    WrongAnswer,
    /// Tempo limite excedido.
    TimeLimitExceeded,
    /// Limite de memória excedido.
    MemoryLimitExceeded,
    /// Erro de apresentação.
    PresentationError,
    /// Erro de compilação.
    CompilationError,
    /// Erro de execução.
    RuntimeError,
}

/// Papel de um membro dentro de uma participação de time.
///
/// Espelha o tipo PostgreSQL `role` e é usado pelas queries para contar apenas
/// participantes competidores quando necessário.
#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "role")]
pub enum Role {
    /// Participante competidor.
    Contestant,
    /// Técnico ou orientador do time.
    Coach,
    /// Reserva vinculado ao time.
    Reserve,
}

/// Nível de uma localização na hierarquia geográfica.
///
/// Espelha o tipo PostgreSQL `location_type` e é usado para filtros e
/// agregações por localidade.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type, PartialEq, Eq, PartialOrd, Ord)]
#[sqlx(type_name = "location_type")]
pub enum LocationType {
    /// Continente.
    Continent,
    /// País.
    Country,
    /// Região.
    Region,
    /// Província, estado ou unidade administrativa equivalente.
    Province,
    /// Prefeitura ou subdivisão administrativa local.
    Prefecture,
    /// Cidade.
    City,
    /// Campus ou unidade institucional.
    Campus,
}

/// Escopo competitivo de um evento.
///
/// Espelha o tipo PostgreSQL `scope` e classifica o alcance geográfico ou
/// institucional de eventos.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type, PartialEq, Eq, PartialOrd, Ord)]
#[sqlx(type_name = "scope")]
pub enum Scope {
    /// Escopo global.
    Global,
    /// Escopo intercontinental.
    InterContinental,
    /// Escopo continental.
    Continental,
    /// Escopo internacional.
    International,
    /// Escopo nacional.
    National,
    /// Escopo inter-regional.
    InterRegional,
    /// Escopo regional.
    Regional,
    /// Escopo interno a uma instituição ou organização.
    Internal,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn location_type_uses_stable_json_variant_names() {
        let serialized = serde_json::to_string(&LocationType::Country).unwrap();
        let deserialized: LocationType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(serialized, "\"Country\"");
        assert_eq!(deserialized, LocationType::Country);
    }

    #[test]
    fn scope_uses_stable_json_variant_names() {
        let serialized = serde_json::to_string(&Scope::InterRegional).unwrap();
        let deserialized: Scope = serde_json::from_str(&serialized).unwrap();

        assert_eq!(serialized, "\"InterRegional\"");
        assert_eq!(deserialized, Scope::InterRegional);
    }

    #[test]
    fn location_type_order_matches_hierarchy_order() {
        let mut values = vec![
            LocationType::City,
            LocationType::Country,
            LocationType::Continent,
        ];

        values.sort();

        assert_eq!(
            values,
            vec![
                LocationType::Continent,
                LocationType::Country,
                LocationType::City,
            ]
        );
    }
}
