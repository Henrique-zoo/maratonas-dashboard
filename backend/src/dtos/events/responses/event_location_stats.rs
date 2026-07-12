//! # `backend::dtos::events::responses::event_location_stats`
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
//! - `EventLocationStats`: DTO de saída serializado em JSON no contrato público da API.
//!
use serde::Serialize;

use crate::repositories::types::events::EventLocationStatsRow;

/// Estatísticas de eventos agrupadas por localização.
///
/// Representa o resumo analítico retornado pelos endpoints de eventos quando
/// a consulta é segmentada por uma localização da hierarquia cadastrada.
#[derive(Default, Serialize, Debug)]
pub struct EventLocationStats {
    /// Identificador da localização agrupada.
    pub id: i32,
    /// Nome da localização exibido no payload público.
    pub name: String,
    /// Total de instituições associadas aos eventos dessa localização.
    pub total_institutions: u32,
    /// Total de times associados aos eventos dessa localização.
    pub total_teams: u32,
    /// Total de participantes associados aos eventos dessa localização.
    pub total_participants: u32,
    /// Total de participantes femininas associadas aos eventos dessa localização.
    pub female_participants: u32,
}

impl From<EventLocationStatsRow> for EventLocationStats {
    /// Converte a projeção SQL de estatísticas por localização em DTO público.
    ///
    /// Os totais vindos do banco como `i32` são normalizados para `u32` no
    /// contrato serializado.
    fn from(value: EventLocationStatsRow) -> Self {
        Self {
            id: value.location_id,
            name: value.location_name,
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
    fn event_location_stats_maps_location_and_totals() {
        let stats = EventLocationStats::from(EventLocationStatsRow {
            location_id: 3,
            location_name: "Sao Paulo".to_string(),
            total_institutions: 9,
            total_teams: 21,
            total_participants: 63,
            female_participants: 18,
        });

        assert_eq!(stats.id, 3);
        assert_eq!(stats.name, "Sao Paulo");
        assert_eq!(stats.total_institutions, 9);
        assert_eq!(stats.total_teams, 21);
        assert_eq!(stats.total_participants, 63);
        assert_eq!(stats.female_participants, 18);
    }
}
