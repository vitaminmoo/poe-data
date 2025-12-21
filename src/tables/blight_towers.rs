#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_BlightTowers: LazyLock<Vec<BlightTowersRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/blighttowers.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| BlightTowersRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#description: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#icon: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(24..24 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#next_upgrade_options: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 8)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values
                    .into_iter()
                    .map(|value| BlightTowersRef::new(value as usize))
                    .collect()
            },
            r#unknown48: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(48..48 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#tier: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(52..52 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#radius: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(60..60 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown64: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(64..64 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#spend_resource_achievement: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(68..68 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                AchievementItemsRef::new(value as usize)
            },
            r#stats_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(84..84 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#stats_keys: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(100..100 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values
                    .into_iter()
                    .map(|value| StatsRef::new(value as usize))
                    .collect()
            },
            r#stats_keys2: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(116..116 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values
                    .into_iter()
                    .map(|value| StatsRef::new(value as usize))
                    .collect()
            },
            r#unknown132: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(132).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct BlightTowersRow {
    pub r#id: String,
    pub r#name: String,
    pub r#description: String,
    pub r#icon: String,
    pub r#next_upgrade_options: Vec<BlightTowersRef>,
    pub r#unknown48: i32,
    pub r#tier: String,
    pub r#radius: i32,
    pub r#unknown64: i32,
    pub r#spend_resource_achievement: AchievementItemsRef,
    pub r#stats_key: StatsRef,
    pub r#stats_keys: Vec<StatsRef>,
    pub r#stats_keys2: Vec<StatsRef>,
    pub r#unknown132: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BlightTowersRef(pub usize);

impl Deref for BlightTowersRef {
    type Target = BlightTowersRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_BlightTowers[self.0]
    }
}

impl BlightTowersRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BlightTowersRow {
        &TABLE_BlightTowers[self.0]
    }
    pub fn get(&self) -> &'static BlightTowersRow {
        &TABLE_BlightTowers[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BlightTowers.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BlightTowersRow)> {
        TABLE_BlightTowers
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
        for row in TABLE_BlightTowers.iter() {
            black_box(row);
        }
    }
}
