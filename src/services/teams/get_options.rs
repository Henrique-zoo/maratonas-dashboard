//! # `backend::services::teams::get_options`
//!
//! ## Responsabilidade
//! Implementa casos de uso do domínio `teams`.
//!
//! ## Lógica de Implementação
//! Valida entrada, consulta traits de repositório e converte dados para DTOs de resposta.
//!
//! ## Funções
//! - `get_options`: Caso de uso de domínio que valida parâmetros e orquestra consulta/transformação de dados.
//!
//! ## Tipos
//! Este módulo não define tipos novos; ele reutiliza contratos declarados em outros arquivos.
//!
use crate::{dtos::common::responses::OptionItem, errors::AppResult, repositories::TeamRepository};

/// Lista opções de times para filtros da API.
///
/// Pode aplicar filtro combinado por competições e instituições antes de
/// converter para `OptionItem`.
///
/// # Parâmetros
/// - `repo`: contrato de acesso a dados de times.
/// - `competition_ids`: filtro opcional por competições.
/// - `institution_ids`: filtro opcional por instituições.
///
/// # Retorno
/// - `Ok(Vec<OptionItem>)` com pares de ID e nome de times.
///
/// # Erros
/// - Propaga erros do repositório.
///
/// # Exemplos
/// ```ignore
/// use backend::services;
/// use backend::errors::AppResult;
/// use backend::repositories::TeamRepository;
///
/// async fn run(repo: &dyn TeamRepository) -> AppResult<()> {
///     let options = services::teams::get_options(repo, Some(vec![10]), Some(vec![5])).await?;
///     assert!(options.iter().all(|item| !item.name.is_empty()));
///     Ok(())
/// }
/// ```
pub async fn get_options(
    repo: &dyn TeamRepository,
    competition_ids: Option<Vec<i32>>,
    institution_ids: Option<Vec<i32>>,
) -> AppResult<Vec<OptionItem>> {
    let options = repo
        .find_options_by_competitions_and_instructions(competition_ids, institution_ids)
        .await?
        .into_iter()
        .map(OptionItem::from)
        .collect();

    Ok(options)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        errors::AppError,
        repositories::{MockTeamRepository, types::IdNameRow},
    };

    #[tokio::test]
    async fn get_options_maps_repository_rows() {
        let mut repo = MockTeamRepository::new();
        repo.expect_find_options_by_competitions_and_instructions()
            .with(
                mockall::predicate::eq(Some(vec![10])),
                mockall::predicate::eq(Some(vec![5])),
            )
            .returning(|_, _| {
                Ok(vec![IdNameRow {
                    id: 1000,
                    name: "Bit Masters".to_string(),
                }])
            });

        let result = get_options(&repo, Some(vec![10]), Some(vec![5]))
            .await
            .unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, 1000);
        assert_eq!(result[0].name, "Bit Masters");
    }

    #[tokio::test]
    async fn get_options_returns_empty_when_repository_returns_empty() {
        let mut repo = MockTeamRepository::new();
        repo.expect_find_options_by_competitions_and_instructions()
            .with(mockall::predicate::eq(None), mockall::predicate::eq(None))
            .returning(|_, _| Ok(vec![]));

        let result = get_options(&repo, None, None).await.unwrap();

        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn get_options_propagates_repository_error() {
        let mut repo = MockTeamRepository::new();
        repo.expect_find_options_by_competitions_and_instructions()
            .with(
                mockall::predicate::eq(Some(vec![10])),
                mockall::predicate::eq(Some(vec![5])),
            )
            .returning(|_, _| Err(AppError::BadRequest("repo fail".to_string())));

        let result = get_options(&repo, Some(vec![10]), Some(vec![5])).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Bad request: repo fail");
    }
}
