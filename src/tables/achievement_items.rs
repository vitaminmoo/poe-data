#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_AchievementItems: LazyLock<Vec<AchievementItemsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/achievementitems.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| AchievementItemsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown8: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown12: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(12..12 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#completions_required: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#achievements_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(28..28 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                AchievementsRef::new(value as usize)
            },
            r#unknown44: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(44).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown45: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(45).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown46: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(46).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown47: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(47).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct AchievementItemsRow {
    pub r#id: String,
    pub r#unknown8: i32,
    pub r#unknown12: i32,
    pub r#name: String,
    pub r#completions_required: i32,
    pub r#achievements_key: AchievementsRef,
    pub r#unknown44: bool,
    pub r#unknown45: bool,
    pub r#unknown46: bool,
    pub r#unknown47: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct AchievementItemsRef(pub usize);

impl Deref for AchievementItemsRef {
    type Target = AchievementItemsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_AchievementItems[self.0]
    }
}

impl AchievementItemsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static AchievementItemsRow {
        &TABLE_AchievementItems[self.0]
    }
    pub fn get(&self) -> &'static AchievementItemsRow {
        &TABLE_AchievementItems[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_AchievementItems.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static AchievementItemsRow)> {
        TABLE_AchievementItems.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_AchievementItems.iter() {
            black_box(row);
        }
    }
}
