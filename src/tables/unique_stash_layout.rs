#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_UniqueStashLayout: LazyLock<Vec<UniqueStashLayoutRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/uniquestashlayout.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| UniqueStashLayoutRow {
            r#words_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                WordsRef::new(value as usize)
            },
            r#item_visual_identity_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ItemVisualIdentityRef::new(value as usize)
            },
            r#unique_stash_types_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                UniqueStashTypesRef::new(value as usize)
            },
            r#unknown48: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(48..48 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown52: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(52..52 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#override_width: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(56..56 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#override_height: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(60..60 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#show_if_empty_challenge_league: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(64).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#show_if_empty_standard: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(65).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#renamed_version: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(66..66 + 8).unwrap();
                let value = cell_bytes.get_i64_le();
                UniqueStashLayoutRef::new(value as usize)
            },
            r#base_version: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(74..74 + 8).unwrap();
                let value = cell_bytes.get_i64_le();
                UniqueStashLayoutRef::new(value as usize)
            },
            r#is_alternate_art: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(82).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct UniqueStashLayoutRow {
    pub r#words_key: WordsRef,
    pub r#item_visual_identity_key: ItemVisualIdentityRef,
    pub r#unique_stash_types_key: UniqueStashTypesRef,
    pub r#unknown48: i32,
    pub r#unknown52: i32,
    pub r#override_width: i32,
    pub r#override_height: i32,
    pub r#show_if_empty_challenge_league: bool,
    pub r#show_if_empty_standard: bool,
    pub r#renamed_version: UniqueStashLayoutRef,
    pub r#base_version: UniqueStashLayoutRef,
    pub r#is_alternate_art: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct UniqueStashLayoutRef(pub usize);

impl Deref for UniqueStashLayoutRef {
    type Target = UniqueStashLayoutRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_UniqueStashLayout[self.0]
    }
}

impl UniqueStashLayoutRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static UniqueStashLayoutRow {
        &TABLE_UniqueStashLayout[self.0]
    }
    pub fn get(&self) -> &'static UniqueStashLayoutRow {
        &TABLE_UniqueStashLayout[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_UniqueStashLayout.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static UniqueStashLayoutRow)> {
        TABLE_UniqueStashLayout.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_UniqueStashLayout.iter() {
            black_box(row);
        }
    }
}
