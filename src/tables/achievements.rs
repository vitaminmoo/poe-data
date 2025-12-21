#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_Achievements: LazyLock<Vec<AchievementsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/achievements.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| AchievementsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#description: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#set_id: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#objective: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(20..20 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#hash16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(28..28 + 2).unwrap();
                let value = cell_bytes.get_i16_le();
                value
            },
            r#unknown30: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(30).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#hide_achievement_items: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(31).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown32: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(32).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#min_completed_items: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(33..33 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#two_column_layout: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(37).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#show_item_completions_as_one: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(38).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown39: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(39..39 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#softcore_only: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(47).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#hardcore_only: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(48).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown49: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(49).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown50: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(50..50 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown58: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(58..58 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown66: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(66..66 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct AchievementsRow {
    pub r#id: String,
    pub r#description: String,
    pub r#set_id: i32,
    pub r#objective: String,
    pub r#hash16: i16,
    pub r#unknown30: bool,
    pub r#hide_achievement_items: bool,
    pub r#unknown32: bool,
    pub r#min_completed_items: i32,
    pub r#two_column_layout: bool,
    pub r#show_item_completions_as_one: bool,
    pub r#unknown39: String,
    pub r#softcore_only: bool,
    pub r#hardcore_only: bool,
    pub r#unknown49: bool,
    pub r#unknown50: String,
    pub r#unknown58: String,
    pub r#unknown66: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct AchievementsRef(pub usize);

impl Deref for AchievementsRef {
    type Target = AchievementsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_Achievements[self.0]
    }
}

impl AchievementsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static AchievementsRow {
        &TABLE_Achievements[self.0]
    }
    pub fn get(&self) -> &'static AchievementsRow {
        &TABLE_Achievements[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Achievements.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static AchievementsRow)> {
        TABLE_Achievements
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
        for row in TABLE_Achievements.iter() {
            black_box(row);
        }
    }
}
