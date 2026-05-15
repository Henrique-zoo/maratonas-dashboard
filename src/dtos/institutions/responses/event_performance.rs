//! # `backend::dtos::institutions::responses::event_performance`
//!
//! ## Responsabilidade
//! Define DTOs de saída do domínio `institutions`.
//!
//! ## Lógica de Implementação
//! Define payloads serializáveis da API e conversões de estruturas internas para JSON estável.
//!
//! ## Funções
//! - `from`: Função de transformação usada na montagem de DTOs de request/response.
//!
//! ## Tipos
//! - `EventPerformance`: DTO de saída serializado em JSON no contrato público da API.
//!
use serde::Serialize;

use crate::repositories::types::institutions::EventPerformanceRow;

/// Desempenho anual de uma instituição em um evento.
///
/// Este DTO compõe a série temporal retornada para comparar a melhor colocação
/// da instituição e sua colocação média em cada ano consultado.
#[derive(Debug, Serialize)]
pub struct EventPerformance {
    /// Ano da medição de desempenho.
    pub year: i32,
    /// Melhor colocação alcançada pela instituição no ano.
    pub best_performance_rank: i32,
    /// Identificador do time responsável pela melhor colocação.
    pub best_performance_team_id: i32,
    /// Nome do time responsável pela melhor colocação.
    pub best_performance_team_name: String,
    /// Colocação média dos times da instituição no ano.
    pub medium_performance_rank: f32,
}

impl From<EventPerformanceRow> for EventPerformance {
    /// Converte uma linha de desempenho do repositório em DTO público.
    ///
    /// A conversão preserva os valores calculados pela query, incluindo melhor
    /// colocação e média anual de performance.
    fn from(value: EventPerformanceRow) -> Self {
        Self {
            year: value.year,
            best_performance_rank: value.best_performance_rank,
            best_performance_team_id: value.best_performance_team_id,
            best_performance_team_name: value.best_performance_team_name,
            medium_performance_rank: value.medium_performance_rank,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_performance_preserves_repository_metrics() {
        let performance = EventPerformance::from(EventPerformanceRow {
            year: 2024,
            best_performance_rank: 1,
            best_performance_team_id: 1000,
            best_performance_team_name: "Bit Masters".to_string(),
            medium_performance_rank: 2.5,
        });

        assert_eq!(performance.year, 2024);
        assert_eq!(performance.best_performance_rank, 1);
        assert_eq!(performance.best_performance_team_id, 1000);
        assert_eq!(performance.best_performance_team_name, "Bit Masters");
        assert_eq!(performance.medium_performance_rank, 2.5);
    }
}
