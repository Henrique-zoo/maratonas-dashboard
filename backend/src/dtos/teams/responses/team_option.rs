//! # `backend::dtos::teams::responses::team_option`
//!
//! Contrato público das opções de equipe.

use serde::Serialize;

use crate::repositories::types::teams::TeamOptionRow;

/// Equipe selecionável acompanhada da instituição à qual pertence.
///
/// `name` permanece o nome canônico da equipe. Os campos institucionais dão ao
/// cliente contexto suficiente para construir rótulos inequívocos quando a
/// seleção abrange mais de uma instituição.
#[derive(Debug, Serialize)]
pub struct TeamOption {
    pub id: i32,
    pub name: String,
    pub institution_id: i32,
    pub institution_name: String,
    pub institution_short_name: Option<String>,
}

impl From<TeamOptionRow> for TeamOption {
    fn from(row: TeamOptionRow) -> Self {
        Self {
            id: row.id,
            name: row.name,
            institution_id: row.institution_id,
            institution_name: row.institution_name,
            institution_short_name: row.institution_short_name,
        }
    }
}
