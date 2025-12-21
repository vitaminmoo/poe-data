#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_UniqueStashTypes: LazyLock<Vec<UniqueStashTypesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/uniquestashtypes.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| UniqueStashTypesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#order: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#width: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(12..12 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#height: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#total_count: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown24: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(28..28 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#standard_count: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(36..36 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#image: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(40..40 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#challenge_league_count: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(48..48 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#is_disabled: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(52).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct UniqueStashTypesRow {
    pub r#id: String,
    pub r#order: i32,
    pub r#width: i32,
    pub r#height: i32,
    pub r#total_count: i32,
    pub r#unknown24: i32,
    pub r#name: String,
    pub r#standard_count: i32,
    pub r#image: String,
    pub r#challenge_league_count: i32,
    pub r#is_disabled: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct UniqueStashTypesRef(pub usize);

impl Deref for UniqueStashTypesRef {
    type Target = UniqueStashTypesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_UniqueStashTypes[self.0]
    }
}

impl UniqueStashTypesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static UniqueStashTypesRow {
        &TABLE_UniqueStashTypes[self.0]
    }
    pub fn get(&self) -> &'static UniqueStashTypesRow {
        &TABLE_UniqueStashTypes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_UniqueStashTypes
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static UniqueStashTypesRow)> {
        TABLE_UniqueStashTypes
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
        for row in TABLE_UniqueStashTypes.iter() {
            black_box(row);
        }
    }
}
