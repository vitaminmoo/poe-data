#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MonsterMapDifficulty: LazyLock<Vec<MonsterMapDifficultyRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/monstermapdifficulty.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| MonsterMapDifficultyRow {
                r#map_level: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(0..0 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#life_percent_increase: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(4..4 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#damage_percent_increase: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(8..8 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#stat1: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(12..12 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
                r#stat2: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(28..28 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
                r#stat3: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(44..44 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
                r#stat3_value: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(60..60 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#stat4: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(64..64 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
                r#stat4_value: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(80..80 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct MonsterMapDifficultyRow {
    pub r#map_level: i32,
    pub r#life_percent_increase: i32,
    pub r#damage_percent_increase: i32,
    pub r#stat1: StatsRef,
    pub r#stat2: StatsRef,
    pub r#stat3: StatsRef,
    pub r#stat3_value: i32,
    pub r#stat4: StatsRef,
    pub r#stat4_value: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MonsterMapDifficultyRef(pub usize);

impl Deref for MonsterMapDifficultyRef {
    type Target = MonsterMapDifficultyRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MonsterMapDifficulty[self.0]
    }
}

impl MonsterMapDifficultyRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MonsterMapDifficultyRow {
        &TABLE_MonsterMapDifficulty[self.0]
    }
    pub fn get(&self) -> &'static MonsterMapDifficultyRow {
        &TABLE_MonsterMapDifficulty[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MonsterMapDifficulty
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MonsterMapDifficultyRow)> {
        TABLE_MonsterMapDifficulty
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
        for row in TABLE_MonsterMapDifficulty.iter() {
            black_box(row);
        }
    }
}
