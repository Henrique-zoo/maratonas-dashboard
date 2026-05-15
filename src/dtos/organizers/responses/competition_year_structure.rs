//! # `backend::dtos::organizers::responses::competition_year_structure`
//!
//! ## Responsabilidade
//! Define DTOs de saída do domínio `organizers`.
//!
//! ## Lógica de Implementação
//! Define payloads serializáveis da API e conversões de estruturas internas para JSON estável.
//!
//! ## Funções
//! - `update`: Função de transformação usada na montagem de DTOs de request/response.
//!
//! ## Tipos
//! - `CompetitionYearStructure`: DTO de saída serializado em JSON no contrato público da API.
//!
use serde::Serialize;

use crate::shared::types::LocationType;

use super::EventSubStructure;

/// Estrutura anual de eventos de uma competição no domínio de organizadores.
///
/// É o payload público retornado quando a visão de organizadores detalha uma
/// competição específica em determinado ano.
#[derive(Default, Debug, Serialize)]
pub struct CompetitionYearStructure {
    /// Tipos de localização presentes nos eventos retornados.
    pub location_types: Vec<LocationType>,
    /// Eventos realizados pela competição no ano consultado.
    pub events: Vec<EventSubStructure>,
}

impl CompetitionYearStructure {
    /// Atualiza os tipos de localização coletados para a competição.
    ///
    /// # Parâmetros
    /// - `location_types`: tipos de localização derivados das linhas do
    ///   repositório.
    pub fn update(&mut self, location_types: Vec<LocationType>) {
        self.location_types = location_types;
    }
}
