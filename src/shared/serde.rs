//! # `backend::shared::serde`
//!
//! ## Responsabilidade
//! Implementa utilitários de desserialização customizada para filtros HTTP.
//!
//! ## Lógica de Implementação
//! Aceita entrada em formatos alternativos (CSV e vetor), normaliza valores e devolve tipos internos consistentes.
//!
//! ## Funções
//! - `deserialize`: Desserializa entrada CSV ou vetor, descartando tokens vazios e validando cada valor.
//! - `into_inner`: Extrai o valor encapsulado sem transformação adicional.
//!
//! ## Tipos
//! - `CsvOptVec`: Wrapper para aceitar filtros opcionais como CSV ou vetor em query params.
//! - `CsvOrVec`: Enum que modela variações semânticas relevantes para o domínio.
//!
use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, de};

/// Formatos aceitos pelo desserializador de lista flexível.
///
/// Permite que uma query seja enviada tanto como string CSV quanto como vetor
/// nativo desserializado pelo `serde`.
#[derive(Deserialize)]
#[serde(untagged)]
enum CsvOrVec<T> {
    /// Valor recebido como string separada por vírgulas.
    Csv(String),
    /// Valor recebido diretamente como vetor.
    Vec(Vec<T>),
}

/// Wrapper para filtros opcionais aceitos como CSV ou vetor.
///
/// É usado em DTOs de query para permitir entradas como `ids=1,2,3` sem
/// espalhar parsing manual pelas camadas HTTP e de serviço.
#[derive(Debug)]
pub struct CsvOptVec<T>(Option<Vec<T>>);

impl<T> Default for CsvOptVec<T> {
    /// Cria um filtro ausente.
    ///
    /// Permite que DTOs de query usem `#[serde(default)]` para interpretar a
    /// ausência do campo como `None`.
    fn default() -> Self {
        Self(None)
    }
}

impl<'de, T> Deserialize<'de> for CsvOptVec<T>
where
    T: Deserialize<'de> + FromStr,
    T::Err: Display,
{
    /// Desserializa uma lista opcional a partir de CSV ou vetor.
    ///
    /// Strings vazias ou compostas apenas por separadores resultam em `None`;
    /// valores não vazios são aparados, convertidos com [`FromStr`] e
    /// retornados como `Some(Vec<T>)`.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match CsvOrVec::deserialize(deserializer)? {
            CsvOrVec::Vec(vec) => Ok(CsvOptVec(Some(vec))),
            CsvOrVec::Csv(s) => {
                let vec = s
                    .split(',')
                    .filter(|s| !s.trim().is_empty())
                    .map(|v| {
                        v.trim()
                            .parse::<T>()
                            .map_err(<D::Error as de::Error>::custom)
                    })
                    .collect::<Result<Vec<T>, _>>()?;

                Ok(CsvOptVec((!vec.is_empty()).then_some(vec)))
            }
        }
    }
}

impl<T> CsvOptVec<T> {
    /// Extrai a lista opcional normalizada.
    ///
    /// # Retorno
    /// `Some(Vec<T>)` quando a query recebeu ao menos um valor válido, ou
    /// `None` quando o filtro não foi informado ou ficou vazio após o parsing.
    pub fn into_inner(self) -> Option<Vec<T>> {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct Query {
        #[serde(default)]
        ids: CsvOptVec<i32>,
    }

    #[test]
    fn csv_opt_vec_parses_trimmed_csv_values() {
        let query: Query = serde_urlencoded::from_str("ids=1,%202,%20%203").unwrap();

        assert_eq!(query.ids.into_inner(), Some(vec![1, 2, 3]));
    }

    #[test]
    fn csv_opt_vec_treats_empty_csv_as_absent_filter() {
        let query: Query = serde_urlencoded::from_str("ids=,,%20").unwrap();

        assert_eq!(query.ids.into_inner(), None);
    }

    #[test]
    fn csv_opt_vec_defaults_missing_query_field_to_none() {
        let query: Query = serde_urlencoded::from_str("").unwrap();

        assert_eq!(query.ids.into_inner(), None);
    }

    #[test]
    fn csv_opt_vec_rejects_invalid_tokens() {
        let result = serde_urlencoded::from_str::<Query>("ids=1,nope,3");

        assert!(result.is_err());
    }
}
