//! # `backend::repositories::event::trait_def`
//!
//! ## Responsabilidade
//! Define o contrato de persistência do domínio `event`.
//!
//! ## Lógica de Implementação
//! Declara trait assíncrona com operações de leitura necessárias aos services, permitindo mock em testes e desacoplamento da implementação SQL.
//!
//! ## Funções
//! - `find_location_stats`: Executa query SQL tipada para recuperar projeções usadas pela camada de serviço.
//! - `find_event_stats_by_year`: Executa query SQL tipada para recuperar projeções usadas pela camada de serviço.
//!
//! ## Tipos
//! - `EventRepository`: Trait que define o contrato de leitura do domínio para desacoplar serviços de SQL.
//!
use async_trait::async_trait;

use crate::{
    errors::AppResult,
    repositories::{
        Registry,
        event::stats,
        types::events::{EventLocationStatsRow, EventYearStatsRow},
    },
    shared::types::LocationType,
};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
/// Contrato de leitura analítica para o domínio de eventos.
///
/// A implementação concreta em [`Registry`] delega para `event::stats`.
pub trait EventRepository: Send + Sync {
    /// Retorna estatísticas de um evento agregadas por localidade.
    ///
    /// # Parâmetros
    /// * `event_id` - ID do evento.
    /// * `location_type` - Nível geográfico para agrupamento.
    /// * `year` - Ano de referência.
    ///
    /// # Retorno
    /// Vetor com totais por localidade, ordenado por `location_name`.
    ///
    /// # Erros
    /// Propaga falhas de acesso ao banco de dados.
    async fn find_location_stats(
        &self,
        event_id: i32,
        location_type: LocationType,
        year: i32,
    ) -> AppResult<Vec<EventLocationStatsRow>>;

    /// Retorna os totais anuais consolidados de um evento.
    ///
    /// # Parâmetros
    /// * `event_id` - ID do evento.
    /// * `year` - Ano de referência.
    ///
    /// # Retorno
    /// Uma linha com totais anuais de instituições, times e participantes.
    ///
    /// # Erros
    /// Propaga falhas de acesso ao banco.
    /// Pode retornar erro de linha ausente quando não há dados para
    /// `(event_id, year)`.
    async fn find_event_stats_by_year(
        &self,
        event_id: i32,
        year: i32,
    ) -> AppResult<EventYearStatsRow>;
}

#[async_trait]
impl EventRepository for Registry {
    /// Implementa [`EventRepository::find_location_stats`].
    ///
    /// Delega o cálculo das estatísticas por localização para
    /// [`stats::find_location_stats`].
    async fn find_location_stats(
        &self,
        event_id: i32,
        location_type: LocationType,
        year: i32,
    ) -> AppResult<Vec<EventLocationStatsRow>> {
        stats::find_location_stats(self, event_id, location_type, year).await
    }

    /// Implementa [`EventRepository::find_event_stats_by_year`].
    ///
    /// Delega o cálculo dos totais anuais para
    /// [`stats::find_event_stats_by_year`].
    async fn find_event_stats_by_year(
        &self,
        event_id: i32,
        year: i32,
    ) -> AppResult<EventYearStatsRow> {
        stats::find_event_stats_by_year(self, event_id, year).await
    }
}
