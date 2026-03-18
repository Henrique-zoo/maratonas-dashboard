use crate::{
    errors::AppResult,
    repositories::{Registry, types::IdNameRow},
};

pub(super) async fn find_options_by_competitions(
    repo: &Registry,
    competition_ids: Option<Vec<i32>>,
) -> AppResult<Vec<IdNameRow>> {
    let rows = if let Some(ids) = competition_ids {
        sqlx::query_as(
            "SELECT DISTINCT
                i.id AS id,
                i.name AS name
            FROM institution i
            JOIN team t ON t.institution_id = i.id
            JOIN team_event te ON te.team_id = t.id
            JOIN event_instance ei ON ei.id = te.event_instance_id
            JOIN event e ON e.id = ei.event_id
            JOIN competition c ON e.competition_id = c.id
            WHERE c.id = ANY($1::int[])
            ORDER BY i.name",
        )
        .bind(ids)
        .fetch_all(&repo.pool)
        .await?
    } else {
        sqlx::query_as(
            "SELECT
                id, name
            FROM institution
            ORDER BY name",
        )
        .fetch_all(&repo.pool)
        .await?
    };

    Ok(rows)
}
