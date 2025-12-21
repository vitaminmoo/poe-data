#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_TalismanMonsterMods: LazyLock<Vec<TalismanMonsterModsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/talismanmonstermods.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| TalismanMonsterModsRow {
            r#mod_type_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ModTypeRef::new(value as usize)
            },
            r#mods_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ModsRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct TalismanMonsterModsRow {
    pub r#mod_type_key: ModTypeRef,
    pub r#mods_key: ModsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct TalismanMonsterModsRef(pub usize);

impl Deref for TalismanMonsterModsRef {
    type Target = TalismanMonsterModsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_TalismanMonsterMods[self.0]
    }
}

impl TalismanMonsterModsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static TalismanMonsterModsRow {
        &TABLE_TalismanMonsterMods[self.0]
    }
    pub fn get(&self) -> &'static TalismanMonsterModsRow {
        &TABLE_TalismanMonsterMods[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_TalismanMonsterMods
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static TalismanMonsterModsRow)> {
        TABLE_TalismanMonsterMods
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
        for row in TABLE_TalismanMonsterMods.iter() {
            black_box(row);
        }
    }
}
