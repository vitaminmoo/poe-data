#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_SanctumPersistentEffectCategories: LazyLock<Vec<SanctumPersistentEffectCategoriesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/sanctumpersistenteffectcategories.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| SanctumPersistentEffectCategoriesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#frame: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#popup: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#glow: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(24..24 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#curse: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(32).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#boon: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(33).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#icon: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(34..34 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(42..42 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#deferral: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(50).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct SanctumPersistentEffectCategoriesRow {
    pub r#id: String,
    pub r#frame: String,
    pub r#popup: String,
    pub r#glow: String,
    pub r#curse: bool,
    pub r#boon: bool,
    pub r#icon: String,
    pub r#name: String,
    pub r#deferral: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct SanctumPersistentEffectCategoriesRef(pub usize);

impl Deref for SanctumPersistentEffectCategoriesRef {
    type Target = SanctumPersistentEffectCategoriesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_SanctumPersistentEffectCategories[self.0]
    }
}

impl SanctumPersistentEffectCategoriesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static SanctumPersistentEffectCategoriesRow {
        &TABLE_SanctumPersistentEffectCategories[self.0]
    }
    pub fn get(&self) -> &'static SanctumPersistentEffectCategoriesRow {
        &TABLE_SanctumPersistentEffectCategories[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_SanctumPersistentEffectCategories.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static SanctumPersistentEffectCategoriesRow)> {
        TABLE_SanctumPersistentEffectCategories.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_SanctumPersistentEffectCategories.iter() {
            black_box(row);
        }
    }
}
