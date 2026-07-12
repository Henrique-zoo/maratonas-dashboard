//! # `backend::errors`
//!
//! ## Responsabilidade
//! Padroniza erros da aplicação e a forma como eles são convertidos em respostas HTTP.
//!
//! ## Lógica de Implementação
//! Modela erros de validação e persistência com `AppError`, usa `AppResult<T>` para propagação e implementa `IntoResponse` para serializar payload JSON com status adequado.
//!
//! ## Funções
//! - `into_response`: Converte o erro para uma resposta HTTP padronizada com status code e JSON.
//!
//! ## Tipos
//! - `AppError`: Enum central de erros da aplicação com variantes para validação e banco.
//! - `AppResult`: Alias de resultado usado para propagar `AppError` de forma uniforme.
//!
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

/// Erro padronizado da aplicação.
///
/// Centraliza falhas de validação e de persistência para que controllers e
/// services propaguem um único tipo de erro até a conversão HTTP.
#[derive(Debug, Error)]
pub enum AppError {
    /// Requisição inválida detectada pela camada de validação ou serviço.
    #[error("Bad request: {0}")]
    BadRequest(String),
    /// Falha propagada pelo driver ou pool PostgreSQL.
    #[error("Database error")]
    Database(#[from] sqlx::Error),
}

/// Resultado padrão usado pelas camadas internas do backend.
///
/// Alias que reduz repetição nas assinaturas e garante que erros retornados
/// por services e repositórios possam ser convertidos em resposta HTTP por
/// [`AppError`].
pub type AppResult<T> = Result<T, AppError>;

impl IntoResponse for AppError {
    /// Converte um erro da aplicação em resposta HTTP JSON.
    ///
    /// `BadRequest` é mapeado para `400 Bad Request`; erros de banco são
    /// mapeados para `500 Internal Server Error`.
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Database(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;

    async fn response_payload(response: Response) -> serde_json::Value {
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        serde_json::from_slice(&body).unwrap()
    }

    #[tokio::test]
    async fn bad_request_response_uses_400_and_public_message() {
        let response = AppError::BadRequest("invalid filter".to_string()).into_response();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        assert_eq!(
            response_payload(response).await,
            json!({ "error": "invalid filter" })
        );
    }

    #[tokio::test]
    async fn database_response_uses_500_and_driver_message() {
        let response = AppError::Database(sqlx::Error::RowNotFound).into_response();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(
            response_payload(response).await,
            json!({ "error": "no rows returned by a query that expected to return at least one row" })
        );
    }
}
