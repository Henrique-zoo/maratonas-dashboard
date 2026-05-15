//! # `backend::services::institutions::get_options`
//!
//! ## Responsabilidade
//! Implementa casos de uso do domínio `institutions`.
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
use crate::{
    dtos::common::responses::OptionItem, errors::AppResult, repositories::InstitutionRepository,
};

/// Lista opções de instituições para uso em filtros da API.
///
/// Pode restringir o resultado às instituições que participam das competições
/// fornecidas.
///
/// # Parâmetros
/// - `repo`: contrato de acesso a dados de instituições.
/// - `competition_ids`: filtro opcional por competições.
///
/// # Retorno
/// - `Ok(Vec<OptionItem>)` com pares de ID e nome de instituição.
///
/// # Erros
/// - Propaga erros do repositório.
///
/// # Exemplos
/// ```ignore
/// use backend::services;
/// use backend::errors::AppResult;
/// use backend::repositories::InstitutionRepository;
///
/// async fn run(repo: &dyn InstitutionRepository) -> AppResult<()> {
///     let options = services::institutions::get_options(repo, Some(vec![10])).await?;
///     assert!(options.iter().all(|item| item.id > 0));
///     Ok(())
/// }
/// ```
pub async fn get_options(
    repo: &dyn InstitutionRepository,
    competition_ids: Option<Vec<i32>>,
) -> AppResult<Vec<OptionItem>> {
    let options = repo
        .find_options_by_competitions(competition_ids)
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
        repositories::{MockInstitutionRepository, types::IdNameRow},
    };

    #[tokio::test]
    async fn get_options_maps_repository_rows() {
        let mut repo = MockInstitutionRepository::new();
        repo.expect_find_options_by_competitions()
            .with(mockall::predicate::eq(Some(vec![10])))
            .returning(|_| {
                Ok(vec![IdNameRow {
                    id: 5,
                    name: "UFRJ".to_string(),
                }])
            });

        let result = get_options(&repo, Some(vec![10])).await.unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, 5);
        assert_eq!(result[0].name, "UFRJ");
    }

    #[tokio::test]
    async fn get_options_returns_empty_when_repository_returns_empty() {
        let mut repo = MockInstitutionRepository::new();
        repo.expect_find_options_by_competitions()
            .with(mockall::predicate::eq(None))
            .returning(|_| Ok(vec![]));

        let result = get_options(&repo, None).await.unwrap();

        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn get_options_propagates_repository_error() {
        let mut repo = MockInstitutionRepository::new();
        repo.expect_find_options_by_competitions()
            .with(mockall::predicate::eq(Some(vec![10])))
            .returning(|_| Err(AppError::BadRequest("repo fail".to_string())));

        let result = get_options(&repo, Some(vec![10])).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Bad request: repo fail");
    }
}
