#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MonsterHeights: LazyLock<Vec<MonsterHeightsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/monsterheights.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| MonsterHeightsRow {
            r#monster_variety: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MonsterVarietiesRef::new(value as usize)
            },
            r#unknown16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#monster_height_bracket: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MonsterHeightBracketsRef::new(value as usize)
            },
            r#unknown36: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(36..36 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown40: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MonsterHeightsRow {
    pub r#monster_variety: MonsterVarietiesRef,
    pub r#unknown16: f32,
    pub r#monster_height_bracket: MonsterHeightBracketsRef,
    pub r#unknown36: i32,
    pub r#unknown40: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MonsterHeightsRef(pub usize);

impl Deref for MonsterHeightsRef {
    type Target = MonsterHeightsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MonsterHeights[self.0]
    }
}

impl MonsterHeightsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MonsterHeightsRow {
        &TABLE_MonsterHeights[self.0]
    }
    pub fn get(&self) -> &'static MonsterHeightsRow {
        &TABLE_MonsterHeights[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MonsterHeights
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MonsterHeightsRow)> {
        TABLE_MonsterHeights
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
        for row in TABLE_MonsterHeights.iter() {
            black_box(row);
        }
    }
}
