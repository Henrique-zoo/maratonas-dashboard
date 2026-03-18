use sqlx::{Postgres, QueryBuilder};

use crate::{
    errors::AppResult,
    repositories::{Registry, types::IdNameRow},
};

pub(super) async fn find_options_by_competitions_and_instructions(
    repo: &Registry,
    competition_ids: Option<Vec<i32>>,
    institution_ids: Option<Vec<i32>>,
) -> AppResult<Vec<IdNameRow>> {
    let mut builder = QueryBuilder::<Postgres>::new(
        "SELECT DISTINCT
            t.id AS id,
            t.name AS name
        FROM team t",
    );

    let mut first = true;
    if let Some(ids) = competition_ids {
        builder.push(
            "JOIN team_event te
                ON te.team_id = t.id
            JOIN event_instance ei
                ON te.event_instance_id = ei.id
            JOIN event e
                ON ei.event_id = e.id ",
        );
        builder
            .push("WHERE e.competition_id = ANY(")
            .push_bind(ids)
            .push(") ");
        first = false;
    }

    if let Some(ids) = institution_ids {
        builder.push(if first { "WHERE " } else { "AND " });
        builder
            .push("t.institution_id = ANY(")
            .push_bind(ids)
            .push(") ");
    }

    builder.push("ORDER BY t.name");

    let rows = builder.build_query_as().fetch_all(&repo.pool).await?;

    Ok(rows)
}
