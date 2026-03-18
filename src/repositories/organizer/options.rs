use crate::{
    errors::AppResult,
    repositories::{Registry, types::IdNameRow},
};

pub(super) async fn find_options(repo: &Registry) -> AppResult<Vec<IdNameRow>> {
    let rows = sqlx::query_as(
        "SELECT
                id, name
            FROM organizer
            ORDER BY name",
    )
    .fetch_all(&repo.pool)
    .await?;

    Ok(rows)
}
