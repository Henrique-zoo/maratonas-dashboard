//! # `backend::repositories::types::institutions::event_performance_row`
//!
//! ## Responsabilidade
//! Define projeções de consulta para o domínio `institutions`.
//!
//! ## Lógica de Implementação
//! Modela linhas retornadas por `sqlx::query_as`, preservando colunas agregadas usadas pelos serviços para transformação.
//!
//! ## Funções
//! Este arquivo não declara funções de produção neste escopo.
//!
//! ## Tipos
//! - `EventPerformanceRow`: Struct de projeção SQL (`FromRow`) usada internamente pelos services.
//!
use sqlx::prelude::FromRow;

/// Linha de desempenho anual de uma instituição em um evento.
///
/// Representa uma posição da série histórica calculada pelo repositório de
/// instituições, incluindo melhor resultado do ano e média de desempenho.
#[derive(FromRow)]
pub struct EventPerformanceRow {
    /// Ano da medição de desempenho.
    pub year: i32,
    /// Melhor colocação alcançada pela instituição no ano.
    pub best_performance_rank: i32,
    /// Identificador do time que alcançou a melhor colocação.
    pub best_performance_team_id: i32,
    /// Nome do time que alcançou a melhor colocação.
    pub best_performance_team_name: String,
    /// Média das colocações dos times da instituição no ano.
    pub medium_performance_rank: f32,
}
