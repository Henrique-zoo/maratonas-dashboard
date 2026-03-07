use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, de};

#[derive(Deserialize)]
#[serde(untagged)]
enum CsvOrVec<T> {
    Csv(String),
    Vec(Vec<T>),
}

#[derive(Debug)]
pub struct CsvOptVec<T>(Option<Vec<T>>);

impl <'de, T> Deserialize<'de> for CsvOptVec<T>
where
    T: Deserialize<'de> + FromStr,
    T::Err: Display
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
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
    pub fn into_inner(self) -> Option<Vec<T>> {
        self.0
    }
}