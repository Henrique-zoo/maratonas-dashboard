//! # `backend::repositories::types::organizers::organizer_structure`
//!
//! ## Responsabilidade
//! Define projeções de consulta para o domínio `organizers`.
//!
//! ## Lógica de Implementação
//! Modela linhas retornadas por `sqlx::query_as`, preservando colunas agregadas usadas pelos serviços para transformação.
//!
//! ## Funções
//! Este arquivo não declara funções de produção neste escopo.
//!
//! ## Tipos
//! - `OrganizerStructureRow`: Struct de projeção SQL (`FromRow`) usada internamente pelos services.
//!
use chrono::NaiveDate;
use sqlx::FromRow;

use crate::shared::types::{GenderCategory, LocationType};

/// Linha de estrutura de organizador.
///
/// Representa um evento do último ano disponível de uma competição organizada,
/// com metadados do organizador, da competição e totais agregados do evento.
#[derive(FromRow)]
pub struct OrganizerStructureRow {
    /// Identificador do organizador.
    pub organizer_id: i32,
    /// Nome do organizador.
    pub organizer_name: String,
    /// URL pública do organizador, quando cadastrada.
    pub organizer_website_url: Option<String>,

    /// Identificador da competição organizada.
    pub competition_id: i32,
    /// Nome da competição organizada.
    pub competition_name: String,
    /// URL pública da competição, quando cadastrada.
    pub competition_website_url: Option<String>,
    /// Categoria de gênero atendida pela competição.
    pub competition_gender_category: GenderCategory,
    /// Anos em que a competição possui eventos registrados.
    pub competition_years: Vec<i32>,
    /// Tipos de localização presentes na competição.
    pub competition_location_types: Vec<LocationType>,

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
    /// Total de instituições distintas no evento.
    pub event_total_institutions: i32,
    /// Total de times distintos no evento.
    pub event_total_teams: i32,
    /// Total de participantes no evento.
    pub event_total_participants: i32,
    /// Total de participantes femininas no evento.
    pub event_female_participants: i32,
    /// Tipos de localização presentes nas equipes do evento.
    pub event_location_types: Vec<LocationType>,
}
