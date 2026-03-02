use axum::{ routing::get, Router };
use sqlx::postgres::PgPoolOptions;
use std::env::{self, VarError};
use anyhow::Context;

use md_backend::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_url = env::var("DATABASE_URL")
        .map_err(|e| match e {
            VarError::NotPresent => anyhow::anyhow!(e).context("DATABASE_URL is not set or contains invalid characters ('=' or '\\0') in its name."),
            VarError::NotUnicode(_) => anyhow::anyhow!(e).context("DATABASE_URL contains an invalid UTF-8 value.")
        })?;

    let pool = PgPoolOptions::new()
        .connect(&db_url).await
        .context("Failed to connect to DB.")?;
    
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Migrations failed");

    let state = AppState::new(pool);

    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap(),
        Router::new()
            .route("/organizer/options", get(handlers::organizers::get_options))
            .route("/competitions/options", get(handlers::competitions::get_options))
            .route("/institutions/options", get(handlers::institutions::get_options))
            .route("/teams/options", get(handlers::teams::get_options))
            .with_state(state)
    ).await?;

    Ok(())
}
