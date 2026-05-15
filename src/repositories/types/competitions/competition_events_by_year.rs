//! # `backend::repositories::types::competitions::competition_events_by_year`
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
//! - `CompetitionEventsByYearRow`: Struct de projeção SQL (`FromRow`) usada internamente pelos services.
//!
use chrono::NaiveDate;
use sqlx::prelude::FromRow;

use crate::shared::types::LocationType;

/// Linha de evento anual de uma competição.
///
/// Representa uma instância de evento já agregada com totais e tipos de
/// localização para a tela anual de competições.
#[derive(FromRow)]
pub struct CompetitionEventsByYearRow {
    /// Tipos de localização presentes na competição no ano consultado.
    pub competition_location_types: Vec<LocationType>,

    /// Identificador do evento.
    pub event_id: i32,
    /// Nome do evento.
    pub event_name: String,
    /// Nível competitivo do evento, quando informado.
    pub event_level: Option<i32>,
    /// Data da instância do evento.
    pub event_date: NaiveDate,
    /// Localização textual da instância do evento.
    pub event_location: String,
    /// Total de instituições distintas no evento.
    pub event_total_institutions: i32,
    /// Total de times distintos no evento.
    pub event_total_teams: i32,
    /// Total de participantes no evento.
    pub event_total_participants: i32,
    /// Total de participantes femininas no evento.
    pub event_female_participants: i32,
    /// Tipos de localização presentes nas equipes do evento.
    pub event_location_types: Vec<LocationType>,
}
