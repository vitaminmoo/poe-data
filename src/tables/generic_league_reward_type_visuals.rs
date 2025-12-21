#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_GenericLeagueRewardTypeVisuals: LazyLock<Vec<GenericLeagueRewardTypeVisualsRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/genericleaguerewardtypevisuals.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| GenericLeagueRewardTypeVisualsRow {
                r#type: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(0..0 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    GenericLeagueRewardTypesRef::new(value as usize)
                },
                r#unknown16: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(16..16 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    value
                },
                r#unknown32: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(32..32 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    value
                },
                r#unknown48: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(48..48 + 4).unwrap();
                    let value = cell_bytes.get_f32_le();
                    value
                },
                r#icon: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(52..52 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#name: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(60..60 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct GenericLeagueRewardTypeVisualsRow {
    pub r#type: GenericLeagueRewardTypesRef,
    pub r#unknown16: i64,
    pub r#unknown32: i64,
    pub r#unknown48: f32,
    pub r#icon: String,
    pub r#name: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct GenericLeagueRewardTypeVisualsRef(pub usize);

impl Deref for GenericLeagueRewardTypeVisualsRef {
    type Target = GenericLeagueRewardTypeVisualsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_GenericLeagueRewardTypeVisuals[self.0]
    }
}

impl GenericLeagueRewardTypeVisualsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static GenericLeagueRewardTypeVisualsRow {
        &TABLE_GenericLeagueRewardTypeVisuals[self.0]
    }
    pub fn get(&self) -> &'static GenericLeagueRewardTypeVisualsRow {
        &TABLE_GenericLeagueRewardTypeVisuals[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_GenericLeagueRewardTypeVisuals
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs(
    ) -> impl Iterator<Item = (Self, &'static GenericLeagueRewardTypeVisualsRow)> {
        TABLE_GenericLeagueRewardTypeVisuals
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_GenericLeagueRewardTypeVisuals.iter() {
            black_box(row);
        }
    }
}
