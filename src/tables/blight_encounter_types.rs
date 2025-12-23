#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_BlightEncounterTypes: LazyLock<Vec<BlightEncounterTypesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/blightencountertypes.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| BlightEncounterTypesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#icon: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#is_generic: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(16).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#weight: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(17..17 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct BlightEncounterTypesRow {
    pub r#id: String,
    pub r#icon: String,
    pub r#is_generic: bool,
    pub r#weight: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BlightEncounterTypesRef(pub usize);

impl Deref for BlightEncounterTypesRef {
    type Target = BlightEncounterTypesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_BlightEncounterTypes[self.0]
    }
}

impl BlightEncounterTypesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BlightEncounterTypesRow {
        &TABLE_BlightEncounterTypes[self.0]
    }
    pub fn get(&self) -> &'static BlightEncounterTypesRow {
        &TABLE_BlightEncounterTypes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BlightEncounterTypes.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BlightEncounterTypesRow)> {
        TABLE_BlightEncounterTypes.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_BlightEncounterTypes.iter() {
            black_box(row);
        }
    }
}
