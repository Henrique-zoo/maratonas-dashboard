//! # `backend::repositories::competition::trait_def`
//!
//! ## Responsabilidade
//! Define o contrato de persistência do domínio `competition`.
//!
//! ## Lógica de Implementação
//! Declara trait assíncrona com operações de leitura necessárias aos services, permitindo mock em testes e desacoplamento da implementação SQL.
//!
//! ## Funções
//! - `find_options_by_organizers`: Executa query SQL tipada para recuperar projeções usadas pela camada de serviço.
//! - `find_structures_by_ids`: Executa query SQL tipada para recuperar projeções usadas pela camada de serviço.
//! - `find_location_stats_by_competition`: Executa query SQL tipada para recuperar projeções usadas pela camada de serviço.
//! - `find_events_by_year`: Executa query SQL tipada para recuperar projeções usadas pela camada de serviço.
//! - `find_structure_by_year`: Executa query SQL tipada para recuperar projeções usadas pela camada de serviço.
//! - `find_competition_stats_by_year`: Executa query SQL tipada para recuperar projeções usadas pela camada de serviço.
//! - `find_team_result_by_year`: Executa query SQL tipada para recuperar projeções usadas pela camada de serviço.
//!
//! ## Tipos
//! - `CompetitionRepository`: Trait que define o contrato de leitura do domínio para desacoplar serviços de SQL.
//!
use async_trait::async_trait;

use crate::{
    errors::AppResult,
    repositories::{
        Registry,
        competition::{options, stats, structures},
        types::{
            IdNameRow,
            competitions::{
                CompetitionEventsByYearRow, CompetitionLocationStatsRow, CompetitionStructureRow,
                CompetitionTeamYearResultRow, CompetitionYearStatsRow, CompetitionYearStructureRow,
            },
        },
    },
    shared::types::LocationType,
};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
/// Contrato de leitura analítica para o domínio de competições.
///
/// A implementação concreta em [`Registry`] delega para consultas SQL nos
/// módulos `competition::options`, `competition::stats` e
/// `competition::structures`.
///
/// Esta trait é usada pelos services para desacoplar regra de negócio da
/// infraestrutura de persistência e também para permitir mocks em testes.
pub trait CompetitionRepository: Send + Sync {
    /// Lista competições para uso em filtros da API.
    ///
    /// Quando `organizer_ids` é `Some`, restringe o resultado às competições
    /// pertencentes aos organizadores informados. Quando é `None`, retorna
    /// todas as competições.
    ///
    /// # Parâmetros
    /// * `organizer_ids` - IDs opcionais de organizadores.
    ///
    /// # Retorno
    /// Vetor de pares `(id, name)` ordenado por `name` em ordem crescente.
    ///
    /// # Erros
    /// Propaga falhas de acesso ao banco de dados.
    async fn find_options_by_organizers(
        &self,
        organizer_ids: Option<Vec<i32>>,
    ) -> AppResult<Vec<IdNameRow>>;

    /// Busca linhas de estrutura para as competições informadas.
    ///
    /// A consulta usa o último ano disponível de cada competição e retorna
    /// linhas denormalizadas contendo:
    /// - metadados da competição (incluindo anos disponíveis),
    /// - metadados do evento,
    /// - dados de instituição/time e ranking no evento,
    /// - recortes de localização agregados.
    ///
    /// # Parâmetros
    /// * `competition_ids` - IDs de competições alvo.
    ///
    /// # Retorno
    /// Linhas ordenadas por `competition_name`, `event_level`, `event_name` e
    /// `team_rank`.
    ///
    /// # Erros
    /// Propaga falhas de acesso ao banco de dados.
    async fn find_structures_by_ids(
        &self,
        competition_ids: Vec<i32>,
    ) -> AppResult<Vec<CompetitionStructureRow>>;

    /// Calcula estatísticas de competição agregadas por localidade.
    ///
    /// O agrupamento é feito no nível geográfico indicado por `location_type`,
    /// considerando as participações do ano informado.
    ///
    /// # Parâmetros
    /// * `competition_id` - ID da competição.
    /// * `location_type` - Nível geográfico de agregação.
    /// * `year` - Ano de referência.
    ///
    /// # Retorno
    /// Vetor com totais por localidade, ordenado por `location_name`.
    ///
    /// # Erros
    /// Propaga falhas de acesso ao banco de dados.
    async fn find_location_stats_by_competition(
        &self,
        competition_id: i32,
        location_type: LocationType,
        year: i32,
    ) -> AppResult<Vec<CompetitionLocationStatsRow>>;

    /// Lista eventos de uma competição em um ano específico com totais
    /// agregados por evento.
    ///
    /// Além dos totais (instituições, times e participantes), inclui recortes
    /// de localização do evento e da competição no período consultado.
    ///
    /// # Parâmetros
    /// * `competition_id` - ID da competição.
    /// * `year` - Ano de referência.
    ///
    /// # Retorno
    /// Vetor ordenado por `event_level`, `event_date` e `event_name`.
    ///
    /// # Erros
    /// Propaga falhas de acesso ao banco de dados.
    async fn find_events_by_year(
        &self,
        competition_id: i32,
        year: i32,
    ) -> AppResult<Vec<CompetitionEventsByYearRow>>;

    /// Retorna linhas de estrutura detalhada da competição em um ano.
    ///
    /// Cada linha representa um time em um evento e traz dados suficientes para
    /// remontar a árvore `evento -> times` na camada de service.
    ///
    /// # Parâmetros
    /// * `competition_id` - ID da competição.
    /// * `year` - Ano de referência.
    ///
    /// # Retorno
    /// Vetor ordenado por `event_level`, `event_date`, `event_name` e
    /// `team_rank`.
    ///
    /// # Erros
    /// Propaga falhas de acesso ao banco de dados.
    async fn find_structure_by_year(
        &self,
        competition_id: i32,
        year: i32,
    ) -> AppResult<Vec<CompetitionYearStructureRow>>;

    /// Calcula totais anuais consolidados de uma competição.
    ///
    /// # Parâmetros
    /// * `competition_id` - ID da competição.
    /// * `year` - Ano de referência.
    ///
    /// # Retorno
    /// Uma linha com totais anuais de instituições, times e participantes.
    ///
    /// # Erros
    /// Propaga falhas de acesso ao banco.
    /// Pode retornar erro de linha ausente quando não há dados para o
    /// `(competition_id, year)` consultado.
    async fn find_competition_stats_by_year(
        &self,
        competition_id: i32,
        year: i32,
    ) -> AppResult<CompetitionYearStatsRow>;

    /// Retorna o desempenho de um time em uma competição no ano informado.
    ///
    /// A consulta inclui escopo e localização de cada evento disputado, além de
    /// totais de membros e de participantes femininas do time na participação.
    ///
    /// # Parâmetros
    /// * `team_id` - ID do time.
    /// * `competition_id` - ID da competição.
    /// * `year` - Ano de referência.
    ///
    /// # Retorno
    /// Vetor ordenado por `team_event_rank`, `event_level`, `event_name` e
    /// `event_date`.
    ///
    /// # Erros
    /// Propaga falhas de acesso ao banco de dados.
    async fn find_team_result_by_year(
        &self,
        team_id: i32,
        competition_id: i32,
        year: i32,
    ) -> AppResult<Vec<CompetitionTeamYearResultRow>>;
}

#[async_trait]
impl CompetitionRepository for Registry {
    /// Implementa [`CompetitionRepository::find_options_by_organizers`].
    ///
    /// Delega a execução SQL para [`options::find_options_by_organizers`],
    /// preservando o `Registry` como ponto único de acesso ao pool.
    async fn find_options_by_organizers(
        &self,
        organizer_ids: Option<Vec<i32>>,
    ) -> AppResult<Vec<IdNameRow>> {
        options::find_options_by_organizers(self, organizer_ids).await
    }

    /// Implementa [`CompetitionRepository::find_structures_by_ids`].
    ///
    /// Delega a montagem das linhas de estrutura para
    /// [`structures::find_structures_by_ids`].
    async fn find_structures_by_ids(
        &self,
        competition_ids: Vec<i32>,
    ) -> AppResult<Vec<CompetitionStructureRow>> {
        structures::find_structures_by_ids(self, competition_ids).await
    }

    /// Implementa [`CompetitionRepository::find_location_stats_by_competition`].
    ///
    /// Delega o cálculo das estatísticas por localização para
    /// [`stats::find_location_stats_by_competition`].
    async fn find_location_stats_by_competition(
        &self,
        competition_id: i32,
        location_type: LocationType,
        year: i32,
    ) -> AppResult<Vec<CompetitionLocationStatsRow>> {
        stats::find_location_stats_by_competition(self, competition_id, location_type, year).await
    }

    /// Implementa [`CompetitionRepository::find_events_by_year`].
    ///
    /// Delega a consulta de eventos anuais para
    /// [`structures::find_events_by_year`].
    async fn find_events_by_year(
        &self,
        competition_id: i32,
        year: i32,
    ) -> AppResult<Vec<CompetitionEventsByYearRow>> {
        structures::find_events_by_year(self, competition_id, year).await
    }

    /// Implementa [`CompetitionRepository::find_structure_by_year`].
    ///
    /// Delega a consulta da estrutura anual para
    /// [`structures::find_structure_by_year`].
    async fn find_structure_by_year(
        &self,
        competition_id: i32,
        year: i32,
    ) -> AppResult<Vec<CompetitionYearStructureRow>> {
        structures::find_structure_by_year(self, competition_id, year).await
    }

    /// Implementa [`CompetitionRepository::find_competition_stats_by_year`].
    ///
    /// Delega o cálculo dos totais anuais para
    /// [`stats::find_competition_stats_by_year`].
    async fn find_competition_stats_by_year(
        &self,
        competition_id: i32,
        year: i32,
    ) -> AppResult<CompetitionYearStatsRow> {
        stats::find_competition_stats_by_year(self, competition_id, year).await
    }

    /// Implementa [`CompetitionRepository::find_team_result_by_year`].
    ///
    /// Delega a consulta de resultados anuais do time para
    /// [`structures::find_team_result_by_year`].
    async fn find_team_result_by_year(
        &self,
        team_id: i32,
        competition_id: i32,
        year: i32,
    ) -> AppResult<Vec<CompetitionTeamYearResultRow>> {
        structures::find_team_result_by_year(self, team_id, competition_id, year).await
    }
}
