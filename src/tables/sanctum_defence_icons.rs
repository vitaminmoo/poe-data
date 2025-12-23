#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_SanctumDefenceIcons: LazyLock<Vec<SanctumDefenceIconsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/sanctumdefenceicons.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| SanctumDefenceIconsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#stat: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#defence_icon: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(24..24 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#defence_broken_icon: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(32..32 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#broken_stat: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#description: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(56..56 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct SanctumDefenceIconsRow {
    pub r#id: String,
    pub r#stat: StatsRef,
    pub r#defence_icon: String,
    pub r#defence_broken_icon: String,
    pub r#broken_stat: StatsRef,
    pub r#description: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct SanctumDefenceIconsRef(pub usize);

impl Deref for SanctumDefenceIconsRef {
    type Target = SanctumDefenceIconsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_SanctumDefenceIcons[self.0]
    }
}

impl SanctumDefenceIconsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static SanctumDefenceIconsRow {
        &TABLE_SanctumDefenceIcons[self.0]
    }
    pub fn get(&self) -> &'static SanctumDefenceIconsRow {
        &TABLE_SanctumDefenceIcons[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_SanctumDefenceIcons.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static SanctumDefenceIconsRow)> {
        TABLE_SanctumDefenceIcons.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_SanctumDefenceIcons.iter() {
            black_box(row);
        }
    }
}
