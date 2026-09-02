//! Extractors HTTP que convertem falhas de desserialização para o envelope de
//! erro uniforme da API.

use axum::{
    extract::{FromRequestParts, Path, Query},
    http::request::Parts,
};
use serde::de::DeserializeOwned;

use crate::errors::AppError;

/// Extrai parâmetros do caminho e converte rejeições do Axum para `AppError`.
pub(crate) struct ApiPath<T>(pub T);

impl<T, S> FromRequestParts<S> for ApiPath<T>
where
    T: DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Path::<T>::from_request_parts(parts, state)
            .await
            .map(|Path(value)| Self(value))
            .map_err(|_| AppError::BadRequest("Invalid path parameters.".to_string()))
    }
}

/// Extrai parâmetros da consulta e converte rejeições do Axum para `AppError`.
pub(crate) struct ApiQuery<T>(pub T);

impl<T, S> FromRequestParts<S> for ApiQuery<T>
where
    T: DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Query::<T>::from_request_parts(parts, state)
            .await
            .map(|Query(value)| Self(value))
            .map_err(|_| AppError::BadRequest("Invalid query parameters.".to_string()))
    }
}
