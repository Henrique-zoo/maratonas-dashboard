//! # `backend::dtos::events::responses::event_year_stats`
//!
//! ## Responsabilidade
//! Define DTOs de saída do domínio `events`.
//!
//! ## Lógica de Implementação
//! Define payloads serializáveis da API e conversões de estruturas internas para JSON estável.
//!
//! ## Funções
//! - `from`: Função de transformação usada na montagem de DTOs de request/response.
//!
//! ## Tipos
//! - `EventYearStats`: DTO de saída serializado em JSON no contrato público da API.
//!
use serde::Serialize;

use crate::repositories::types::events::EventYearStatsRow;

/// Estatísticas consolidadas de eventos em um ano.
///
/// Expõe os totais anuais calculados pelo repositório já normalizados para
/// inteiros sem sinal no contrato JSON da API.
#[derive(Debug, Serialize)]
pub struct EventYearStats {
    /// Total de instituições participantes no ano.
    pub total_institutions: u32,
    /// Total de times participantes no ano.
    pub total_teams: u32,
    /// Total de participantes no ano.
    pub total_participants: u32,
    /// Total de participantes femininas no ano.
    pub female_participants: u32,
}

impl From<EventYearStatsRow> for EventYearStats {
    /// Converte a projeção SQL de estatísticas anuais em DTO público.
    ///
    /// Os totais vindos do banco como `i32` são normalizados para `u32` no
    /// contrato serializado.
    fn from(value: EventYearStatsRow) -> Self {
        Self {
            total_institutions: value.total_institutions as u32,
            total_teams: value.total_teams as u32,
            total_participants: value.total_participants as u32,
            female_participants: value.female_participants as u32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_year_stats_normalizes_repository_totals() {
        let stats = EventYearStats::from(EventYearStatsRow {
            total_institutions: 8,
            total_teams: 16,
            total_participants: 48,
            female_participants: 14,
        });

        assert_eq!(stats.total_institutions, 8);
        assert_eq!(stats.total_teams, 16);
        assert_eq!(stats.total_participants, 48);
        assert_eq!(stats.female_participants, 14);
    }
}
