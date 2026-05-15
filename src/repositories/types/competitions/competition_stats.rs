//! # `backend::repositories::types::competitions::competition_stats`
//!
//! ## Responsabilidade
//! Define projeções de consulta para o domínio `competitions`.
//!
//! ## Lógica de Implementação
//! Modela linhas retornadas por `sqlx::query_as`, preservando colunas agregadas usadas pelos serviços para transformação.
//!
//! ## Funções
//! Este arquivo não declara funções de produção neste escopo.
//!
//! ## Tipos
//! - `CompetitionLocationStatsRow`: Struct de projeção SQL (`FromRow`) usada internamente pelos services.
//! - `CompetitionYearStatsRow`: Struct de projeção SQL (`FromRow`) usada internamente pelos services.
//!
use sqlx::prelude::FromRow;

/// Estatísticas de competição agregadas por localização.
///
/// Corresponde às colunas selecionadas pelas consultas de estatísticas por
/// localidade no repositório de competições.
#[derive(FromRow)]
pub struct CompetitionLocationStatsRow {
    /// Identificador da localização agrupada.
    pub location_id: i32,
    /// Nome da localização agrupada.
    pub location_name: String,
    /// Total de instituições distintas no recorte.
    pub total_institutions: i32,
    /// Total de times distintos no recorte.
    pub total_teams: i32,
    /// Total de participantes no recorte.
    pub total_participants: i32,
    /// Total de participantes femininas no recorte.
    pub female_participants: i32,
}

/// Estatísticas anuais consolidadas de uma competição.
///
/// Agrega os totais calculados para uma competição em um ano específico antes
/// da conversão para o DTO público.
#[derive(FromRow)]
pub struct CompetitionYearStatsRow {
    /// Total de instituições distintas no ano.
    pub total_institutions: i32,
    /// Total de times distintos no ano.
    pub total_teams: i32,
    /// Total de participantes no ano.
    pub total_participants: i32,
    /// Total de participantes femininas no ano.
    pub female_participants: i32,
}
