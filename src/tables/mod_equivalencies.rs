#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ModEquivalencies: LazyLock<Vec<ModEquivalenciesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/modequivalencies.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ModEquivalenciesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#mods_key0: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ModsRef::new(value as usize)
            },
            r#mods_key1: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ModsRef::new(value as usize)
            },
            r#mods_key2: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ModsRef::new(value as usize)
            },
            r#unknown56: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(56).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ModEquivalenciesRow {
    pub r#id: String,
    pub r#mods_key0: ModsRef,
    pub r#mods_key1: ModsRef,
    pub r#mods_key2: ModsRef,
    pub r#unknown56: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ModEquivalenciesRef(pub usize);

impl Deref for ModEquivalenciesRef {
    type Target = ModEquivalenciesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ModEquivalencies[self.0]
    }
}

impl ModEquivalenciesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ModEquivalenciesRow {
        &TABLE_ModEquivalencies[self.0]
    }
    pub fn get(&self) -> &'static ModEquivalenciesRow {
        &TABLE_ModEquivalencies[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ModEquivalencies
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ModEquivalenciesRow)> {
        TABLE_ModEquivalencies
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
        for row in TABLE_ModEquivalencies.iter() {
            black_box(row);
        }
    }
}
