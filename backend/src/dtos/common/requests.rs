//! # `backend::dtos::common::requests`
//!
//! ## Responsabilidade
//! Agrupa DTOs do domínio `common`.
//!
//! ## Lógica de Implementação
//! Conecta módulos de request/response para compor o contrato HTTP público desse domínio.
//!
//! ## Funções
//! Este arquivo não declara funções de produção neste escopo.
//!
//! ## Tipos
//! - `IdPath`: Struct utilizada para modelar dados deste domínio.
//! - `YearQuery`: Struct utilizada para modelar dados deste domínio.
//! - `LocationYearQuery`: Struct utilizada para modelar dados deste domínio.
//!
use serde::Deserialize;

use crate::shared::types::LocationType;

/// Parâmetros de rota que identificam um recurso por ID numérico.
///
/// Este DTO é usado por handlers que recebem um único identificador no path e
/// repassam esse valor para a camada de serviço sem alterar sua semântica.
#[derive(Debug, Deserialize)]
pub struct IdPath {
    /// Identificador do recurso extraído do path da requisição.
    pub id: i32,
}

/// Parâmetros de query para endpoints filtrados por ano.
///
/// O campo é opcional para permitir que a camada de serviço aplique validação
/// contextual e retorne o erro de domínio apropriado quando o ano for exigido.
#[derive(Debug, Deserialize)]
pub struct YearQuery {
    /// Ano de referência informado na query string.
    pub year: Option<i32>,
}

/// Parâmetros de query para estatísticas segmentadas por localização e ano.
///
/// Combina o tipo de localização com o ano de referência usado nos relatórios
/// analíticos de competições e eventos.
#[derive(Debug, Deserialize)]
pub struct LocationYearQuery {
    /// Tipo de localização usado para agrupar ou filtrar o resultado.
    pub location_type: Option<LocationType>,
    /// Ano de referência usado pela consulta analítica.
    pub year: Option<i32>,
}
