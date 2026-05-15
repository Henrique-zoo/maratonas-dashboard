//! # `backend::services::competitions::get_options`
//!
//! ## Responsabilidade
//! Implementa casos de uso do domínio `competitions`.
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
    dtos::common::responses::OptionItem, errors::AppResult, repositories::CompetitionRepository,
};

/// Lista opções de competições para preenchimento de filtros da API.
///
/// Opcionalmente filtra o resultado pelos organizadores recebidos e converte
/// o resultado para `OptionItem`.
///
/// # Parâmetros
/// - `repo`: contrato de acesso a dados de competições.
/// - `organizer_ids`: IDs de organizadores usados como filtro opcional.
///
/// # Retorno
/// - `Ok(Vec<OptionItem>)` com pares de identificador e nome.
///
/// # Erros
/// - Propaga erros do repositório.
///
/// # Exemplos
/// ```ignore
/// use backend::services;
/// use backend::errors::AppResult;
/// use backend::repositories::CompetitionRepository;
///
/// async fn run(repo: &dyn CompetitionRepository) -> AppResult<()> {
///     let options = services::competitions::get_options(repo, Some(vec![1, 2])).await?;
///     for item in options {
///         println!("{} - {}", item.id, item.name);
///     }
///     Ok(())
/// }
/// ```
pub async fn get_options(
    repo: &dyn CompetitionRepository,
    organizer_ids: Option<Vec<i32>>,
) -> AppResult<Vec<OptionItem>> {
    let options = repo
        .find_options_by_organizers(organizer_ids)
        .await?
        .into_iter()
        .map(OptionItem::from)
        .collect();

    Ok(options)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::types::IdNameRow;
    use crate::{errors::AppError, repositories::MockCompetitionRepository};

    #[tokio::test]
    async fn get_option_returns_filters_from_repository_rows() {
        let mut repo = MockCompetitionRepository::new();

        repo.expect_find_options_by_organizers()
            .with(mockall::predicate::eq(Some(vec![1, 2])))
            .returning(|_| {
                Ok(vec![
                    IdNameRow {
                        id: 1,
                        name: "ICPC Brazil".to_string(),
                    },
                    IdNameRow {
                        id: 2,
                        name: "ICPC LatAm".to_string(),
                    },
                ])
            });

        let result = get_options(&repo, Some(vec![1, 2])).await.unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].id, 1);
        assert_eq!(result[0].name, "ICPC Brazil");
        assert_eq!(result[1].id, 2);
        assert_eq!(result[1].name, "ICPC LatAm");
    }

    #[tokio::test]
    async fn get_option_returns_empty_when_repository_returns_empty() {
        let mut repo = MockCompetitionRepository::new();

        repo.expect_find_options_by_organizers()
            .with(mockall::predicate::eq(None))
            .returning(|_| Ok(vec![]));

        let result = get_options(&repo, None).await.unwrap();

        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn get_option_propagates_repository_error() {
        let mut repo = MockCompetitionRepository::new();

        repo.expect_find_options_by_organizers()
            .returning(|_| Err(AppError::BadRequest("repo fail".to_string())));

        let result = get_options(&repo, Some(vec![1])).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Bad request: repo fail");
    }
}
