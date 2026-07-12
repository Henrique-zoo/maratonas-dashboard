//! # `backend::dtos::common::responses`
//!
//! ## Responsabilidade
//! Agrupa DTOs do domínio `common`.
//!
//! ## Lógica de Implementação
//! Conecta módulos de request/response para compor o contrato HTTP público desse domínio.
//!
//! ## Funções
//! - `from`: Função de transformação usada na montagem de DTOs de request/response.
//!
//! ## Tipos
//! - `OptionItem`: Struct utilizada para modelar dados deste domínio.
//!
use serde::Serialize;

use crate::repositories::types::IdNameRow;

/// Item genérico de opção retornado por endpoints de seleção.
///
/// Representa entidades exibidas em listas compactas, como filtros de
/// competições, instituições, organizadores ou times.
#[derive(Debug, Serialize)]
pub struct OptionItem {
    /// Identificador da entidade selecionável.
    pub id: i32,
    /// Nome exibido para o usuário na lista de opções.
    pub name: String,
}

impl From<IdNameRow> for OptionItem {
    /// Converte uma projeção compacta de repositório em opção pública.
    ///
    /// Preserva `id` e `name` sem transformação adicional.
    fn from(row: IdNameRow) -> Self {
        Self {
            id: row.id,
            name: row.name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_item_preserves_repository_projection() {
        let item = OptionItem::from(IdNameRow {
            id: 42,
            name: "ICPC".to_string(),
        });

        assert_eq!(item.id, 42);
        assert_eq!(item.name, "ICPC");
    }
}
