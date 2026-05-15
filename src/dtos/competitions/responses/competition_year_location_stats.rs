//! # `backend::dtos::competitions::responses::competition_year_location_stats`
//!
//! ## Responsabilidade
//! Define DTOs de saída do domínio `competitions`.
//!
//! ## Lógica de Implementação
//! Define payloads serializáveis da API e conversões de estruturas internas para JSON estável.
//!
//! ## Funções
//! - `from`: Função de transformação usada na montagem de DTOs de request/response.
//!
//! ## Tipos
//! - `CompetitionYearLocationStats`: DTO de saída serializado em JSON no contrato público da API.
//!
use serde::Serialize;

use crate::repositories::types::competitions::CompetitionLocationStatsRow;

/// Estatísticas anuais de uma competição agrupadas por localização.
///
/// Este DTO compõe respostas analíticas que mostram, para cada localização,
/// os totais consolidados de instituições, times e participantes.
#[derive(Debug, Serialize)]
pub struct CompetitionYearLocationStats {
    /// Identificador da localização agrupada.
    pub id: i32,
    /// Nome da localização exibido no payload público.
    pub name: String,
    /// Total de instituições associadas à competição nessa localização.
    pub total_institutions: u32,
    /// Total de times associados à competição nessa localização.
    pub total_teams: u32,
    /// Total de participantes associados à competição nessa localização.
    pub total_participants: u32,
    /// Total de participantes femininas associadas à competição nessa localização.
    pub female_participants: u32,
}

impl From<CompetitionLocationStatsRow> for CompetitionYearLocationStats {
    /// Converte a projeção SQL de estatísticas por localização em DTO público.
    ///
    /// Mapeia `location_id`/`location_name` para `id`/`name` e normaliza os
    /// totais para inteiros sem sinal.
    fn from(value: CompetitionLocationStatsRow) -> Self {
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
    fn competition_location_stats_maps_location_and_totals() {
        let stats = CompetitionYearLocationStats::from(CompetitionLocationStatsRow {
            location_id: 7,
            location_name: "Brazil".to_string(),
            total_institutions: 18,
            total_teams: 42,
            total_participants: 126,
            female_participants: 31,
        });

        assert_eq!(stats.id, 7);
        assert_eq!(stats.name, "Brazil");
        assert_eq!(stats.total_institutions, 18);
        assert_eq!(stats.total_teams, 42);
        assert_eq!(stats.total_participants, 126);
        assert_eq!(stats.female_participants, 31);
    }
}
