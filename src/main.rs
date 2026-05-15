//! # `backend::main`
//!
//! ## Responsabilidade
//! Define o ponto de entrada do backend e prepara a aplicação para atender requisições HTTP.
//!
//! ## Lógica de Implementação
//! Lê variáveis de ambiente, cria o pool PostgreSQL, aplica migrations SQLx, constrói o `AppState` e inicia o servidor Axum.
//!
//! ## Funções
//! - `main`: Inicializa infraestrutura da aplicação e inicia o servidor HTTP.
//!
//! ## Tipos
//! Este módulo não define tipos novos; ele reutiliza contratos declarados em outros arquivos.
//!
use anyhow::Context;
use sqlx::postgres::PgPoolOptions;
use std::env::{self, VarError};

use backend::*;

/// Inicializa e executa o servidor HTTP do backend.
///
/// Carrega a URL do banco a partir de `DATABASE_URL`, abre o pool PostgreSQL,
/// executa as migrations embutidas pelo `sqlx`, constrói o estado global e
/// publica a aplicação Axum em `0.0.0.0:8000`.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_url = env::var("DATABASE_URL").map_err(|e| match e {
        VarError::NotPresent => anyhow::anyhow!(e).context(
            "DATABASE_URL is not set or contains invalid characters ('=' or '\\0') in its name.",
        ),
        VarError::NotUnicode(_) => {
            anyhow::anyhow!(e).context("DATABASE_URL contains an invalid UTF-8 value.")
        }
    })?;

    let pool = PgPoolOptions::new()
        .connect(&db_url)
        .await
        .context("Failed to connect to DB.")?;

    sqlx::migrate!()
        .run(&pool)
        .await
        .context("Migrations failed")?;

    let state = AppState::new(pool);

    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap(),
        routes::create_router().with_state(state),
    )
    .await?;

    Ok(())
}
