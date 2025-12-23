#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MonsterPackEntries: LazyLock<Vec<MonsterPackEntriesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/monsterpackentries.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| MonsterPackEntriesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#monster_packs_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MonsterPacksRef::new(value as usize)
            },
            r#flag: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(24).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#weight: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(25..25 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#monster_varieties_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(29..29 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MonsterVarietiesRef::new(value as usize)
            },
            r#unknown45: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(45..45 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MonsterPackEntriesRow {
    pub r#id: String,
    pub r#monster_packs_key: MonsterPacksRef,
    pub r#flag: bool,
    pub r#weight: i32,
    pub r#monster_varieties_key: MonsterVarietiesRef,
    pub r#unknown45: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MonsterPackEntriesRef(pub usize);

impl Deref for MonsterPackEntriesRef {
    type Target = MonsterPackEntriesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MonsterPackEntries[self.0]
    }
}

impl MonsterPackEntriesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MonsterPackEntriesRow {
        &TABLE_MonsterPackEntries[self.0]
    }
    pub fn get(&self) -> &'static MonsterPackEntriesRow {
        &TABLE_MonsterPackEntries[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MonsterPackEntries.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MonsterPackEntriesRow)> {
        TABLE_MonsterPackEntries.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_MonsterPackEntries.iter() {
            black_box(row);
        }
    }
}
