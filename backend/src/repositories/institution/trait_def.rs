//! # `backend::repositories::institution::trait_def`
//!
//! ## Responsabilidade
//! Define o contrato de persistência do domínio `institution`.
//!
//! ## Lógica de Implementação
//! Declara trait assíncrona com operações de leitura necessárias aos services, permitindo mock em testes e desacoplamento da implementação SQL.
//!
//! ## Funções
//! - `find_options_by_competitions`: Executa query SQL tipada para recuperar projeções usadas pela camada de serviço.
//! - `find_structures_by_ids`: Executa query SQL tipada para recuperar projeções usadas pela camada de serviço.
//! - `find_event_performance_over_time`: Executa query SQL tipada para recuperar projeções usadas pela camada de serviço.
//!
//! ## Tipos
//! - `InstitutionRepository`: Trait que define o contrato de leitura do domínio para desacoplar serviços de SQL.
//!
use async_trait::async_trait;

use crate::{
    errors::AppResult,
    repositories::{
        Registry,
        institution::{options, performance, structures},
        types::{
            IdNameRow,
            institutions::{EventPerformanceRow, InstitutionStructureRow},
        },
    },
};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
/// Contrato de leitura analítica para o domínio de instituições.
///
/// A implementação concreta em [`Registry`] delega para os módulos
/// `institution::options`, `institution::structures` e
/// `institution::performance`.
pub trait InstitutionRepository: Send + Sync {
    /// Lista instituições para composição de filtros na API.
    ///
    /// Quando `competition_ids` é `Some`, retorna apenas instituições que
    /// tiveram participação nas competições informadas. Quando `None`, retorna
    /// todas as instituições.
    ///
    /// # Parâmetros
    /// * `competition_ids` - IDs opcionais de competições.
    ///
    /// # Retorno
    /// Vetor de pares `(id, name)` ordenado por `name`.
    ///
    /// # Erros
    /// Propaga falhas de acesso ao banco de dados.
    async fn find_options_by_competitions(
        &self,
        competition_ids: Option<Vec<i32>>,
    ) -> AppResult<Vec<IdNameRow>>;

    /// Retorna estrutura detalhada das instituições informadas.
    ///
    /// A consulta considera, para cada competição relacionada, apenas o último
    /// ano disponível daquela competição e devolve linhas denormalizadas para
    /// montagem da árvore `instituicao -> competicoes -> eventos -> times`.
    ///
    /// # Parâmetros
    /// * `institution_ids` - IDs das instituições alvo.
    ///
    /// # Retorno
    /// Linhas ordenadas por `institution_name`, `competition_name`,
    /// `event_name` e `team_name`.
    ///
    /// # Erros
    /// Propaga falhas de acesso ao banco de dados.
    async fn find_structures_by_ids(
        &self,
        institution_ids: Vec<i32>,
    ) -> AppResult<Vec<InstitutionStructureRow>>;

    /// Retorna o histórico anual de desempenho de uma instituição em um evento.
    ///
    /// Para cada ano no intervalo informado, a consulta retorna:
    /// - melhor rank alcançado,
    /// - time associado à melhor performance,
    /// - média de ranks da instituição no ano.
    ///
    /// # Parâmetros
    /// * `institution_id` - ID da instituição.
    /// * `event_id` - ID do evento.
    /// * `start_year` - Ano inicial (inclusive).
    /// * `end_year` - Ano final (inclusive).
    ///
    /// # Retorno
    /// Vetor ordenado por `year`.
    ///
    /// # Erros
    /// Propaga falhas de acesso ao banco de dados.
    async fn find_event_performance_over_time(
        &self,
        institution_id: i32,
        event_id: i32,
        start_year: i32,
        end_year: i32,
    ) -> AppResult<Vec<EventPerformanceRow>>;
}

#[async_trait]
impl InstitutionRepository for Registry {
    /// Implementa [`InstitutionRepository::find_options_by_competitions`].
    ///
    /// Delega a execução SQL para [`options::find_options_by_competitions`],
    /// preservando o `Registry` como ponto único de acesso ao pool.
    async fn find_options_by_competitions(
        &self,
        competition_ids: Option<Vec<i32>>,
    ) -> AppResult<Vec<IdNameRow>> {
        options::find_options_by_competitions(self, competition_ids).await
    }

    /// Implementa [`InstitutionRepository::find_structures_by_ids`].
    ///
    /// Delega a montagem das linhas de estrutura para
    /// [`structures::find_structures_by_ids`].
    async fn find_structures_by_ids(
        &self,
        institution_ids: Vec<i32>,
    ) -> AppResult<Vec<InstitutionStructureRow>> {
        structures::find_structures_by_ids(self, institution_ids).await
    }

    /// Implementa [`InstitutionRepository::find_event_performance_over_time`].
    ///
    /// Delega a consulta da série histórica para
    /// [`performance::find_event_performance_over_time`].
    async fn find_event_performance_over_time(
        &self,
        institution_id: i32,
        event_id: i32,
        start_year: i32,
        end_year: i32,
    ) -> AppResult<Vec<EventPerformanceRow>> {
        performance::find_event_performance_over_time(
            self,
            institution_id,
            event_id,
            start_year,
            end_year,
        )
        .await
    }
}
