#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_BetrayalRanks: LazyLock<Vec<BetrayalRanksRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/betrayalranks.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| BetrayalRanksRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#text: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#rank_image: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(20..20 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct BetrayalRanksRow {
    pub r#id: String,
    pub r#text: String,
    pub r#level: i32,
    pub r#rank_image: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BetrayalRanksRef(pub usize);

impl Deref for BetrayalRanksRef {
    type Target = BetrayalRanksRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_BetrayalRanks[self.0]
    }
}

impl BetrayalRanksRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BetrayalRanksRow {
        &TABLE_BetrayalRanks[self.0]
    }
    pub fn get(&self) -> &'static BetrayalRanksRow {
        &TABLE_BetrayalRanks[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BetrayalRanks.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BetrayalRanksRow)> {
        TABLE_BetrayalRanks
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
        for row in TABLE_BetrayalRanks.iter() {
            black_box(row);
        }
    }
}
