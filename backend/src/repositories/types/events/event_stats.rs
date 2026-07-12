//! # `backend::repositories::types::events::event_stats`
//!
//! ## Responsabilidade
//! Define projeções de consulta para o domínio `events`.
//!
//! ## Lógica de Implementação
//! Modela linhas retornadas por `sqlx::query_as`, preservando colunas agregadas usadas pelos serviços para transformação.
//!
//! ## Funções
//! Este arquivo não declara funções de produção neste escopo.
//!
//! ## Tipos
//! - `EventLocationStatsRow`: Struct de projeção SQL (`FromRow`) usada internamente pelos services.
//! - `EventYearStatsRow`: Struct de projeção SQL (`FromRow`) usada internamente pelos services.
//!
use sqlx::prelude::FromRow;

/// Estatísticas de evento agregadas por localização.
///
/// Corresponde às colunas selecionadas pelas consultas de estatísticas por
/// localidade no repositório de eventos.
#[derive(FromRow)]
pub struct EventLocationStatsRow {
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

/// Estatísticas anuais consolidadas de um evento.
///
/// Agrega os totais calculados para um evento em um ano específico antes da
/// conversão para o DTO público.
#[derive(FromRow)]
pub struct EventYearStatsRow {
    /// Total de instituições distintas no ano.
    pub total_institutions: i32,
    /// Total de times distintos no ano.
    pub total_teams: i32,
    /// Total de participantes no ano.
    pub total_participants: i32,
    /// Total de participantes femininas no ano.
    pub female_participants: i32,
}
