#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MonsterMapBossDifficulty: LazyLock<Vec<MonsterMapBossDifficultyRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/monstermapbossdifficulty.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| MonsterMapBossDifficultyRow {
                r#map_level: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(0..0 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#boss_life_percent_increase: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(4..4 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#boss_damage_percent_increase: {
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
                r#boss_inc_item_quantity: {
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
                r#boss_inc_item_rarity: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(80..80 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#stat5: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(84..84 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
                r#boss_ailment_percent_decrease: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(100..100 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct MonsterMapBossDifficultyRow {
    pub r#map_level: i32,
    pub r#boss_life_percent_increase: i32,
    pub r#boss_damage_percent_increase: i32,
    pub r#stat1: StatsRef,
    pub r#stat2: StatsRef,
    pub r#stat3: StatsRef,
    pub r#boss_inc_item_quantity: i32,
    pub r#stat4: StatsRef,
    pub r#boss_inc_item_rarity: i32,
    pub r#stat5: StatsRef,
    pub r#boss_ailment_percent_decrease: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MonsterMapBossDifficultyRef(pub usize);

impl Deref for MonsterMapBossDifficultyRef {
    type Target = MonsterMapBossDifficultyRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MonsterMapBossDifficulty[self.0]
    }
}

impl MonsterMapBossDifficultyRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MonsterMapBossDifficultyRow {
        &TABLE_MonsterMapBossDifficulty[self.0]
    }
    pub fn get(&self) -> &'static MonsterMapBossDifficultyRow {
        &TABLE_MonsterMapBossDifficulty[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MonsterMapBossDifficulty
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MonsterMapBossDifficultyRow)> {
        TABLE_MonsterMapBossDifficulty
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
        for row in TABLE_MonsterMapBossDifficulty.iter() {
            black_box(row);
        }
    }
}
