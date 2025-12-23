#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_AfflictionFixedMods: LazyLock<Vec<AfflictionFixedModsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/afflictionfixedmods.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| AfflictionFixedModsRow {
            r#rarity: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#mod: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(4..4 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ModsRef::new(value as usize)
            },
            r#unknown20: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct AfflictionFixedModsRow {
    pub r#rarity: i32,
    pub r#mod: ModsRef,
    pub r#unknown20: i64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct AfflictionFixedModsRef(pub usize);

impl Deref for AfflictionFixedModsRef {
    type Target = AfflictionFixedModsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_AfflictionFixedMods[self.0]
    }
}

impl AfflictionFixedModsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static AfflictionFixedModsRow {
        &TABLE_AfflictionFixedMods[self.0]
    }
    pub fn get(&self) -> &'static AfflictionFixedModsRow {
        &TABLE_AfflictionFixedMods[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_AfflictionFixedMods.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static AfflictionFixedModsRow)> {
        TABLE_AfflictionFixedMods.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_AfflictionFixedMods.iter() {
            black_box(row);
        }
    }
}
