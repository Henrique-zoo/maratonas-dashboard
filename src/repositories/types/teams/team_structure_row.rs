//! # `backend::repositories::types::teams::team_structure_row`
//!
//! ## Responsabilidade
//! Define projeções de consulta para o domínio `teams`.
//!
//! ## Lógica de Implementação
//! Modela linhas retornadas por `sqlx::query_as`, preservando colunas agregadas usadas pelos serviços para transformação.
//!
//! ## Funções
//! Este arquivo não declara funções de produção neste escopo.
//!
//! ## Tipos
//! - `TeamStructureRow`: Struct de projeção SQL (`FromRow`) usada internamente pelos services.
//!
use chrono::NaiveDate;
use sqlx::FromRow;

use crate::shared::types::{GenderCategory, Scope};

/// Linha de estrutura de time.
///
/// Representa uma participação de um time em um evento do último ano
/// disponível de uma competição, com os metadados necessários para montar a
/// árvore `time -> competições -> eventos`.
#[derive(FromRow)]
pub struct TeamStructureRow {
    /// Identificador do time.
    pub team_id: i32,
    /// Nome do time.
    pub team_name: String,
    /// Total de integrantes do time no evento.
    pub team_total_members: i32,
    /// Total de integrantes femininas do time no evento.
    pub team_female_members: i32,

    /// Identificador da competição.
    pub competition_id: i32,
    /// Nome da competição.
    pub competition_name: String,
    /// URL pública da competição, quando cadastrada.
    pub competition_website_url: Option<String>,
    /// Categoria de gênero atendida pela competição.
    pub competition_gender_category: GenderCategory,
    /// Anos em que o time possui participação na competição.
    pub competition_years: Vec<i32>,

    /// Identificador do evento.
    pub event_id: i32,
    /// Nome do evento.
    pub event_name: String,
    /// Nível competitivo do evento, quando informado.
    pub event_level: Option<i32>,
    /// Data da instância do evento.
    pub event_date: NaiveDate,
    /// Localização textual da instância do evento.
    pub event_location: String,
    /// Escopo competitivo do evento.
    pub event_scope: Scope,
    /// Colocação do time no evento.
    pub team_event_rank: i32,
}
