#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_WorldPopupIconTypes: LazyLock<Vec<WorldPopupIconTypesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/worldpopupicontypes.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| WorldPopupIconTypesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown8: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown16: {
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
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(25..25 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown29: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(29..29 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown33: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(33..33 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown37: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(37..37 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct WorldPopupIconTypesRow {
    pub r#id: String,
    pub r#unknown8: String,
    pub r#unknown16: i32,
    pub r#unknown20: i32,
    pub r#unknown24: bool,
    pub r#unknown25: f32,
    pub r#unknown29: f32,
    pub r#unknown33: f32,
    pub r#unknown37: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct WorldPopupIconTypesRef(pub usize);

impl Deref for WorldPopupIconTypesRef {
    type Target = WorldPopupIconTypesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_WorldPopupIconTypes[self.0]
    }
}

impl WorldPopupIconTypesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static WorldPopupIconTypesRow {
        &TABLE_WorldPopupIconTypes[self.0]
    }
    pub fn get(&self) -> &'static WorldPopupIconTypesRow {
        &TABLE_WorldPopupIconTypes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_WorldPopupIconTypes
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static WorldPopupIconTypesRow)> {
        TABLE_WorldPopupIconTypes
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
        for row in TABLE_WorldPopupIconTypes.iter() {
            black_box(row);
        }
    }
}
