//! # `backend::services::events::get_location_stats`
//!
//! ## Responsabilidade
//! Implementa casos de uso do domínio `events`.
//!
//! ## Lógica de Implementação
//! Valida entrada, consulta traits de repositório e converte dados para DTOs de resposta.
//!
//! ## Funções
//! - `get_location_stats`: Caso de uso de domínio que valida parâmetros e orquestra consulta/transformação de dados.
//!
//! ## Tipos
//! Este módulo não define tipos novos; ele reutiliza contratos declarados em outros arquivos.
//!
use crate::{
    dtos::events::responses::EventLocationStats,
    errors::{AppError, AppResult},
    repositories::EventRepository,
    shared::types::LocationType,
};

/// Retorna estatísticas de um evento agrupadas por localização.
///
/// Exige tipo de localização e ano, delega a consulta ao repositório e mapeia
/// cada linha para o DTO `EventLocationStats`.
///
/// # Parâmetros
/// - `repo`: contrato de acesso a dados de eventos.
/// - `event_id`: ID do evento.
/// - `location_type`: nível geográfico para agregação.
/// - `year`: ano de referência.
///
/// # Retorno
/// - `Ok(Vec<EventLocationStats>)` com totais por localidade.
///
/// # Erros
/// - Retorna `AppError::BadRequest` quando `location_type` ou `year` são
///   ausentes.
/// - Propaga erros do repositório.
///
/// # Exemplos
/// ```ignore
/// use backend::services;
/// use backend::errors::AppResult;
/// use backend::repositories::EventRepository;
/// use backend::shared::types::LocationType;
///
/// async fn run(repo: &dyn EventRepository) -> AppResult<()> {
///     let stats = services::events::get_location_stats(
///         repo,
///         20,
///         Some(LocationType::Country),
///         Some(2024),
///     )
///     .await?;
///     assert!(stats.iter().all(|row| row.total_teams >= 0));
///     Ok(())
/// }
/// ```
pub async fn get_location_stats(
    repo: &dyn EventRepository,
    event_id: i32,
    location_type: Option<LocationType>,
    year: Option<i32>,
) -> AppResult<Vec<EventLocationStats>> {
    let location_type = location_type
        .ok_or_else(|| AppError::BadRequest("You need to specify a location type.".to_string()))?;
    let year =
        year.ok_or_else(|| AppError::BadRequest("You need to specify a year.".to_string()))?;

    let stats = repo
        .find_location_stats(event_id, location_type, year)
        .await?
        .into_iter()
        .map(EventLocationStats::from)
        .collect();

    Ok(stats)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        errors::AppError,
        repositories::{MockEventRepository, types::events::EventLocationStatsRow},
    };

    #[tokio::test]
    async fn get_location_stats_requires_location_type_and_year() {
        let repo = MockEventRepository::new();

        assert!(
            get_location_stats(&repo, 20, None, Some(2024))
                .await
                .is_err()
        );
        assert!(
            get_location_stats(&repo, 20, Some(LocationType::Country), None)
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn get_location_stats_maps_repository_rows() {
        let mut repo = MockEventRepository::new();
        repo.expect_find_location_stats()
            .with(
                mockall::predicate::eq(20),
                mockall::predicate::eq(LocationType::Country),
                mockall::predicate::eq(2024),
            )
            .returning(|_, _, _| {
                Ok(vec![EventLocationStatsRow {
                    location_id: 1,
                    location_name: "Brazil".to_string(),
                    total_institutions: 12,
                    total_teams: 24,
                    total_participants: 72,
                    female_participants: 18,
                }])
            });

        let result = get_location_stats(&repo, 20, Some(LocationType::Country), Some(2024))
            .await
            .unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "Brazil");
        assert_eq!(result[0].female_participants, 18);
    }

    #[tokio::test]
    async fn get_location_stats_returns_empty_when_repository_returns_empty() {
        let mut repo = MockEventRepository::new();
        repo.expect_find_location_stats()
            .returning(|_, _, _| Ok(vec![]));

        let result = get_location_stats(&repo, 20, Some(LocationType::Country), Some(2024))
            .await
            .unwrap();

        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn get_location_stats_propagates_repository_error() {
        let mut repo = MockEventRepository::new();
        repo.expect_find_location_stats()
            .returning(|_, _, _| Err(AppError::BadRequest("repo fail".to_string())));

        let result = get_location_stats(&repo, 20, Some(LocationType::Country), Some(2024)).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Bad request: repo fail");
    }
}
