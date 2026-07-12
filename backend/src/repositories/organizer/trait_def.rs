//! # `backend::repositories::organizer::trait_def`
//!
//! ## Responsabilidade
//! Define o contrato de persistência do domínio `organizer`.
//!
//! ## Lógica de Implementação
//! Declara trait assíncrona com operações de leitura necessárias aos services, permitindo mock em testes e desacoplamento da implementação SQL.
//!
//! ## Funções
//! - `find_options`: Executa query SQL tipada para recuperar projeções usadas pela camada de serviço.
//! - `find_structures_by_ids`: Executa query SQL tipada para recuperar projeções usadas pela camada de serviço.
//!
//! ## Tipos
//! - `OrganizerRepository`: Trait que define o contrato de leitura do domínio para desacoplar serviços de SQL.
//!
use async_trait::async_trait;

use crate::{
    errors::AppResult,
    repositories::{
        Registry,
        organizer::{options, structures},
        types::{IdNameRow, organizers::OrganizerStructureRow},
    },
};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
/// Contrato de leitura analítica para o domínio de organizadores.
///
/// A implementação concreta em [`Registry`] delega para
/// `organizer::options` e `organizer::structures`.
pub trait OrganizerRepository: Send + Sync {
    /// Lista organizadores para preenchimento de filtros na API.
    ///
    /// # Retorno
    /// Vetor de pares `(id, name)` ordenado por `name`.
    ///
    /// # Erros
    /// Propaga falhas de acesso ao banco de dados.
    async fn find_options(&self) -> AppResult<Vec<IdNameRow>>;

    /// Retorna estruturas dos organizadores informados.
    ///
    /// A consulta devolve linhas denormalizadas para montagem da árvore
    /// `organizador -> competicoes -> eventos`, considerando o último ano de
    /// cada competição.
    ///
    /// # Parâmetros
    /// * `organizer_ids` - IDs dos organizadores alvo.
    ///
    /// # Retorno
    /// Linhas ordenadas por `organizer_name`, `competition_id`, `event_level`
    /// e `event_name`.
    ///
    /// # Erros
    /// Propaga falhas de acesso ao banco de dados.
    async fn find_structures_by_ids(
        &self,
        organizer_ids: Vec<i32>,
    ) -> AppResult<Vec<OrganizerStructureRow>>;
}

#[async_trait]
impl OrganizerRepository for Registry {
    /// Implementa [`OrganizerRepository::find_options`].
    ///
    /// Delega a execução SQL para [`options::find_options`], preservando o
    /// `Registry` como ponto único de acesso ao pool.
    async fn find_options(&self) -> AppResult<Vec<IdNameRow>> {
        options::find_options(self).await
    }

    /// Implementa [`OrganizerRepository::find_structures_by_ids`].
    ///
    /// Delega a montagem das linhas de estrutura para
    /// [`structures::find_structures_by_ids`].
    async fn find_structures_by_ids(
        &self,
        organizer_ids: Vec<i32>,
    ) -> AppResult<Vec<OrganizerStructureRow>> {
        structures::find_structures_by_ids(self, organizer_ids).await
    }
}
