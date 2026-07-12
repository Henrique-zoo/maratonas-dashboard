//! # `backend::repositories::types::competitions::competition_team_year_result`
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
//! - `CompetitionTeamYearResultRow`: Struct de projeção SQL (`FromRow`) usada internamente pelos services.
//!
use chrono::NaiveDate;
use sqlx::prelude::FromRow;

use crate::shared::types::Scope;

/// Resultado anual de um time em uma competição.
///
/// Representa uma participação do time em evento da competição no ano
/// consultado, incluindo ranking, escopo, localização e totais da equipe.
#[derive(FromRow)]
pub struct CompetitionTeamYearResultRow {
    /// Total de integrantes do time nessa participação.
    pub team_total_members: i32,
    /// Total de integrantes femininas do time nessa participação.
    pub team_female_members: i32,

    /// Identificador do evento disputado.
    pub event_id: i32,
    /// Nome do evento disputado.
    pub event_name: String,
    /// Nível competitivo do evento, quando informado.
    pub event_level: Option<i32>,
    /// Data da instância do evento.
    pub event_date: NaiveDate,
    /// Localização textual da instância do evento.
    pub event_location: String,
    /// Escopo competitivo do evento.
    pub event_scope: Scope,
    /// Colocação do time no evento.
    pub team_event_rank: i32,
}
