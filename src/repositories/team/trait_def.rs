//! # `backend::repositories::team::trait_def`
//!
//! ## Responsabilidade
//! Define o contrato de persistência do domínio `team`.
//!
//! ## Lógica de Implementação
//! Declara trait assíncrona com operações de leitura necessárias aos services, permitindo mock em testes e desacoplamento da implementação SQL.
//!
//! ## Funções
//! - `find_options_by_competitions_and_instructions`: Executa query SQL tipada para recuperar projeções usadas pela camada de serviço.
//! - `find_structures_by_ids`: Executa query SQL tipada para recuperar projeções usadas pela camada de serviço.
//!
//! ## Tipos
//! - `TeamRepository`: Trait que define o contrato de leitura do domínio para desacoplar serviços de SQL.
//!
use async_trait::async_trait;

use crate::{
    errors::AppResult,
    repositories::{
        Registry,
        team::{options, structures},
        types::{IdNameRow, teams::TeamStructureRow},
    },
};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
/// Contrato de leitura analítica para o domínio de times.
///
/// A implementação concreta em [`Registry`] delega para `team::options` e
/// `team::structures`.
pub trait TeamRepository: Send + Sync {
    /// Lista times para filtros, com combinação opcional de restrições.
    ///
    /// A implementação real aplica filtros por competição e/ou instituição
    /// quando os parâmetros são fornecidos, e retorna apenas times distintos.
    ///
    /// # Parâmetros
    /// * `competition_ids` - IDs opcionais de competições.
    /// * `institution_ids` - IDs opcionais de instituições.
    ///
    /// # Retorno
    /// Vetor de pares `(id, name)` ordenado por `name`.
    ///
    /// # Erros
    /// Propaga falhas de acesso ao banco de dados.
    async fn find_options_by_competitions_and_instructions(
        &self,
        competition_ids: Option<Vec<i32>>,
        institution_ids: Option<Vec<i32>>,
    ) -> AppResult<Vec<IdNameRow>>;

    /// Retorna estrutura detalhada dos times informados.
    ///
    /// Para cada combinação `time + competição`, a consulta considera o último
    /// ano em que o time participou da competição e retorna linhas adequadas
    /// para montagem da árvore `time -> competicoes -> eventos`.
    ///
    /// # Parâmetros
    /// * `team_ids` - IDs dos times alvo.
    ///
    /// # Retorno
    /// Linhas ordenadas por `team_name`, `competition_name`, `team_event_rank`,
    /// `event_level` e `event_name`.
    ///
    /// # Erros
    /// Propaga falhas de acesso ao banco de dados.
    async fn find_structures_by_ids(&self, team_ids: Vec<i32>) -> AppResult<Vec<TeamStructureRow>>;
}

#[async_trait]
impl TeamRepository for Registry {
    /// Implementa [`TeamRepository::find_options_by_competitions_and_instructions`].
    ///
    /// Delega a execução SQL dinâmica para
    /// [`options::find_options_by_competitions_and_instructions`].
    async fn find_options_by_competitions_and_instructions(
        &self,
        competition_ids: Option<Vec<i32>>,
        institution_ids: Option<Vec<i32>>,
    ) -> AppResult<Vec<IdNameRow>> {
        options::find_options_by_competitions_and_instructions(
            self,
            competition_ids,
            institution_ids,
        )
        .await
    }

    /// Implementa [`TeamRepository::find_structures_by_ids`].
    ///
    /// Delega a montagem das linhas de estrutura para
    /// [`structures::find_structures_by_ids`].
    async fn find_structures_by_ids(&self, team_ids: Vec<i32>) -> AppResult<Vec<TeamStructureRow>> {
        structures::find_structures_by_ids(self, team_ids).await
    }
}
