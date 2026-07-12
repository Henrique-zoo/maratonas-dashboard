//! # `backend::repositories::types::competitions::competition_structure`
//!
//! ## Responsabilidade
//! Define projeções de consulta para o domínio `competitions`.
//!
//! ## Lógica de Implementação
//! Modela linhas retornadas por `sqlx::query_as`, preservando colunas agregadas usadas pelos serviços para transformação.
//!
//! ## Funções
//! Este arquivo não declara funções de produção neste escopo.
//!
//! ## Tipos
//! - `CompetitionStructureRow`: Struct de projeção SQL (`FromRow`) usada internamente pelos services.
//! - `CompetitionYearStructureRow`: Struct de projeção SQL (`FromRow`) usada internamente pelos services.
//!
use chrono::NaiveDate;
use sqlx::prelude::FromRow;

use crate::shared::types::{GenderCategory, LocationType};

/// Linha de estrutura completa de competição.
///
/// Representa uma participação de time em evento do último ano disponível da
/// competição, preservando metadados da competição, do evento, da instituição
/// e do time em uma única linha denormalizada.
#[derive(FromRow)]
pub struct CompetitionStructureRow {
    /// Identificador da competição.
    pub competition_id: i32,
    /// Nome da competição.
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
    /// Tipos de localização presentes nas equipes do evento.
    pub event_location_types: Vec<LocationType>,

    /// Nome da instituição vinculada ao time.
    pub institution_name: String,
    /// Nome curto da instituição, quando cadastrado.
    pub institution_short_name: Option<String>,
    /// Localização textual da instituição ou campus usado pela participação.
    pub institution_location: String,

    /// Identificador do time.
    pub team_id: i32,
    /// Nome do time.
    pub team_name: String,
    /// Colocação do time no evento.
    pub team_rank: i32,
    /// Total de integrantes do time no evento.
    pub team_total_members: i32,
    /// Total de integrantes femininas do time no evento.
    pub team_female_members: i32,
}

/// Linha de estrutura anual de competição.
///
/// Representa uma participação de time em evento de uma competição no ano
/// consultado, com os campos necessários para montar a visão detalhada anual.
#[derive(FromRow)]
pub struct CompetitionYearStructureRow {
    /// Tipos de localização presentes na competição no ano consultado.
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
    /// Tipos de localização presentes nas equipes do evento.
    pub event_location_types: Vec<LocationType>,

    /// Nome da instituição vinculada ao time.
    pub institution_name: String,
    /// Nome curto da instituição, quando cadastrado.
    pub institution_short_name: Option<String>,
    /// Localização textual da instituição ou campus usado pela participação.
    pub institution_location: String,

    /// Identificador do time.
    pub team_id: i32,
    /// Nome do time.
    pub team_name: String,
    /// Colocação do time no evento.
    pub team_rank: i32,
    /// Total de integrantes do time no evento.
    pub team_total_members: i32,
    /// Total de integrantes femininas do time no evento.
    pub team_female_members: i32,
}
