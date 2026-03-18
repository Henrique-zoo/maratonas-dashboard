use crate::{
    errors::AppResult,
    repositories::{Registry, types::IdNameRow},
};

pub(super) async fn find_options_by_organizers(
    repo: &Registry,
    organizer_ids: Option<Vec<i32>>,
) -> AppResult<Vec<IdNameRow>> {
    let rows = if let Some(ids) = organizer_ids {
        sqlx::query_as(
            "SELECT
                id, name
            FROM competition
            WHERE organizer_id = ANY($1::int[])
            ORDER BY name",
        )
        .bind(ids)
        .fetch_all(&repo.pool)
        .await?
    } else {
        sqlx::query_as(
            "SELECT
                id, name
            FROM competition
            ORDER BY name",
        )
        .fetch_all(&repo.pool)
        .await?
    };

    Ok(rows)
}
