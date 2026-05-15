//! # `backend::dtos::competitions::responses::competition_year_stats`
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
//! - `CompetitionYearStats`: DTO de saída serializado em JSON no contrato público da API.
//!
use serde::Serialize;

use crate::repositories::types::competitions::CompetitionYearStatsRow;

/// Estatísticas consolidadas de uma competição em um ano.
///
/// Representa o resumo numérico do endpoint anual de competições depois que a
/// linha de repositório já foi convertida para o contrato JSON da API.
#[derive(Debug, Serialize)]
pub struct CompetitionYearStats {
    /// Total de instituições participantes no ano.
    pub total_institutions: u32,
    /// Total de times participantes no ano.
    pub total_teams: u32,
    /// Total de participantes no ano.
    pub total_participants: u32,
    /// Total de participantes femininas no ano.
    pub female_participants: u32,
}

impl From<CompetitionYearStatsRow> for CompetitionYearStats {
    /// Converte a projeção SQL de estatísticas anuais em DTO público.
    ///
    /// Os totais vindos do banco como `i32` são normalizados para `u32` no
    /// contrato serializado.
    fn from(value: CompetitionYearStatsRow) -> Self {
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
    fn competition_year_stats_normalizes_repository_totals() {
        let stats = CompetitionYearStats::from(CompetitionYearStatsRow {
            total_institutions: 12,
            total_teams: 34,
            total_participants: 102,
            female_participants: 27,
        });

        assert_eq!(stats.total_institutions, 12);
        assert_eq!(stats.total_teams, 34);
        assert_eq!(stats.total_participants, 102);
        assert_eq!(stats.female_participants, 27);
    }
}
