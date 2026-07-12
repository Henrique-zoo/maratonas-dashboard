//! # `backend::repositories::types::id_name_row`
//!
//! ## Responsabilidade
//! Define projeções de consulta para o domínio `core`.
//!
//! ## Lógica de Implementação
//! Modela linhas retornadas por `sqlx::query_as`, preservando colunas agregadas usadas pelos serviços para transformação.
//!
//! ## Funções
//! Este arquivo não declara funções de produção neste escopo.
//!
//! ## Tipos
//! - `IdNameRow`: Struct de projeção SQL (`FromRow`) usada internamente pelos services.
//!
/// Projeção compacta de uma entidade identificada por `id` e `name`.
///
/// É usada por consultas de opções, em que os services precisam apenas do
/// identificador e do nome exibível antes de converter a linha para o DTO de
/// resposta.
#[derive(sqlx::FromRow)]
pub struct IdNameRow {
    /// Identificador primário da entidade retornada pela query.
    pub id: i32,
    /// Nome exibível da entidade retornada pela query.
    pub name: String,
}
