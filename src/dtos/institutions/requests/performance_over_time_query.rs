//! # `backend::dtos::institutions::requests::performance_over_time_query`
//!
//! ## Responsabilidade
//! Define DTOs de entrada do domínio `institutions`.
//!
//! ## Lógica de Implementação
//! Modela parâmetros deserializados de query/path para validação e tipagem forte antes da camada de serviço.
//!
//! ## Funções
//! Este arquivo não declara funções de produção neste escopo.
//!
//! ## Tipos
//! - `EventPerformancePath`: DTO de entrada da API para desserializar e tipar parâmetros da requisição.
//! - `EventPerformanceQuery`: DTO de entrada da API para desserializar e tipar parâmetros da requisição.
//!
use serde::Deserialize;

/// Parâmetros de rota para consultar desempenho de uma instituição em um evento.
///
/// Identifica o par instituição/evento usado pela consulta histórica de
/// desempenho ao longo do tempo.
#[derive(Deserialize)]
pub struct EventPerformancePath {
    /// Identificador da instituição analisada.
    pub institution_id: i32,
    /// Identificador do evento usado como referência.
    pub event_id: i32,
}

/// Parâmetros de query para delimitar a série histórica de desempenho.
///
/// Os limites são opcionais para permitir consultas abertas, ficando a
/// validação de obrigatoriedade e consistência na camada de serviço.
#[derive(Deserialize)]
pub struct EventPerformanceQuery {
    /// Primeiro ano incluído na série, quando informado.
    pub start_year: Option<i32>,
    /// Último ano incluído na série, quando informado.
    pub end_year: Option<i32>,
}
