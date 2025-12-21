#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_Strongboxes: LazyLock<Vec<StrongboxesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/strongboxes.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| StrongboxesRow {
            r#chests_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ChestsRef::new(value as usize)
            },
            r#spawn_weight: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown20: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown24: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(24).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown25: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(25).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#spawn_weight_increase: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(26..26 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#spawn_weight_hardmode: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(42..42 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct StrongboxesRow {
    pub r#chests_key: ChestsRef,
    pub r#spawn_weight: i32,
    pub r#unknown20: i32,
    pub r#unknown24: bool,
    pub r#unknown25: bool,
    pub r#spawn_weight_increase: StatsRef,
    pub r#spawn_weight_hardmode: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct StrongboxesRef(pub usize);

impl Deref for StrongboxesRef {
    type Target = StrongboxesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_Strongboxes[self.0]
    }
}

impl StrongboxesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static StrongboxesRow {
        &TABLE_Strongboxes[self.0]
    }
    pub fn get(&self) -> &'static StrongboxesRow {
        &TABLE_Strongboxes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Strongboxes.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static StrongboxesRow)> {
        TABLE_Strongboxes
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
        for row in TABLE_Strongboxes.iter() {
            black_box(row);
        }
    }
}
