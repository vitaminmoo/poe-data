#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_CloneShot: LazyLock<Vec<CloneShotRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/cloneshot.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| CloneShotRow {
            r#id: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#monster_varieties_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(4..4 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MonsterVarietiesRef::new(value as usize)
            },
            r#misc_animated1: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MiscAnimatedRef::new(value as usize)
            },
            r#misc_animated2: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(36..36 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MiscAnimatedRef::new(value as usize)
            },
            r#misc_animated3: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(52..52 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MiscAnimatedRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct CloneShotRow {
    pub r#id: i32,
    pub r#monster_varieties_key: MonsterVarietiesRef,
    pub r#misc_animated1: MiscAnimatedRef,
    pub r#misc_animated2: MiscAnimatedRef,
    pub r#misc_animated3: MiscAnimatedRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct CloneShotRef(pub usize);

impl Deref for CloneShotRef {
    type Target = CloneShotRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_CloneShot[self.0]
    }
}

impl CloneShotRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static CloneShotRow {
        &TABLE_CloneShot[self.0]
    }
    pub fn get(&self) -> &'static CloneShotRow {
        &TABLE_CloneShot[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_CloneShot.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static CloneShotRow)> {
        TABLE_CloneShot
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
        for row in TABLE_CloneShot.iter() {
            black_box(row);
        }
    }
}
