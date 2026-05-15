//! # `backend::repositories::types::institutions::institution_structure_row`
//!
//! ## Responsabilidade
//! Define projeções de consulta para o domínio `institutions`.
//!
//! ## Lógica de Implementação
//! Modela linhas retornadas por `sqlx::query_as`, preservando colunas agregadas usadas pelos serviços para transformação.
//!
//! ## Funções
//! Este arquivo não declara funções de produção neste escopo.
//!
//! ## Tipos
//! - `InstitutionStructureRow`: Struct de projeção SQL (`FromRow`) usada internamente pelos services.
//!
use chrono::NaiveDate;
use sqlx::prelude::FromRow;

use crate::shared::types::Scope;

/// Linha de estrutura de instituição.
///
/// Representa uma participação de time de uma instituição em evento do último
/// ano disponível de uma competição, com dados denormalizados para montar a
/// árvore `instituição -> competições -> eventos -> times`.
#[derive(FromRow)]
pub struct InstitutionStructureRow {
    /// Identificador da instituição.
    pub institution_id: i32,
    /// Nome da instituição.
    pub institution_name: String,
    /// Nome curto da instituição, quando cadastrado.
    pub institution_short_name: Option<String>,
    /// Localização textual principal da instituição.
    pub institution_location: String,

    /// Identificador da competição.
    pub competition_id: i32,
    /// Nome da competição.
    pub competition_name: String,
    /// URL pública da competição, quando cadastrada.
    pub competition_website_url: Option<String>,

    /// Identificador do evento.
    pub event_id: i32,
    /// Nome do evento.
    pub event_name: String,
    /// Data da instância do evento.
    pub event_date: NaiveDate,
    /// Nível competitivo do evento, quando informado.
    pub event_level: Option<i32>,
    /// Escopo competitivo do evento.
    pub event_scope: Scope,

    /// Identificador do time da instituição.
    pub team_id: i32,
    /// Nome do time da instituição.
    pub team_name: String,
    /// Colocação do time no evento.
    pub team_event_rank: i32,
    /// Total de integrantes do time no evento.
    pub team_total_members: i32,
    /// Total de integrantes femininas do time no evento.
    pub team_female_members: i32,
}
