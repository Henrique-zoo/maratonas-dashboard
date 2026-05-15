//! # `backend::dtos::teams::responses::competition_year_structure`
//!
//! ## Responsabilidade
//! Define DTOs de saída do domínio `teams`.
//!
//! ## Lógica de Implementação
//! Define payloads serializáveis da API e conversões de estruturas internas para JSON estável.
//!
//! ## Funções
//! Este arquivo não declara funções de produção neste escopo.
//!
//! ## Tipos
//! - `CompetitionYearStructure`: DTO de saída serializado em JSON no contrato público da API.
//!
use serde::Serialize;

use super::EventSubStructure;

/// Estrutura anual de um time em uma competição.
///
/// Resume os totais do time e os eventos em que ele participou no ano
/// consultado.
#[derive(Default, Debug, Serialize)]
pub struct CompetitionYearStructure {
    /// Total de integrantes do time no recorte anual.
    pub total_members: u32,
    /// Total de integrantes femininas do time no recorte anual.
    pub female_participants: u32,
    /// Eventos disputados pelo time no ano consultado.
    pub events: Vec<EventSubStructure>,
}
