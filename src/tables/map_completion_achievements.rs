#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MapCompletionAchievements: LazyLock<Vec<MapCompletionAchievementsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/mapcompletionachievements.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| MapCompletionAchievementsRow {
            r#unknown0: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#map_stat_conditions_keys: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| MapStatConditionsRef::new(value as usize)).collect()
            },
            r#stats_keys: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| StatsRef::new(value as usize)).collect()
            },
            r#achievement_items_keys: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(40..40 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| AchievementItemsRef::new(value as usize)).collect()
            },
            r#map_tier_achievements_keys: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(56..56 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| MapTierAchievementsRef::new(value as usize)).collect()
            },
            r#unknown72: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(72).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#world_areas_keys: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(73..73 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| WorldAreasRef::new(value as usize)).collect()
            },
            r#unknown89: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(89..89 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MapCompletionAchievementsRow {
    pub r#unknown0: String,
    pub r#map_stat_conditions_keys: Vec<MapStatConditionsRef>,
    pub r#stats_keys: Vec<StatsRef>,
    pub r#achievement_items_keys: Vec<AchievementItemsRef>,
    pub r#map_tier_achievements_keys: Vec<MapTierAchievementsRef>,
    pub r#unknown72: bool,
    pub r#world_areas_keys: Vec<WorldAreasRef>,
    pub r#unknown89: (usize, usize),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MapCompletionAchievementsRef(pub usize);

impl Deref for MapCompletionAchievementsRef {
    type Target = MapCompletionAchievementsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MapCompletionAchievements[self.0]
    }
}

impl MapCompletionAchievementsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MapCompletionAchievementsRow {
        &TABLE_MapCompletionAchievements[self.0]
    }
    pub fn get(&self) -> &'static MapCompletionAchievementsRow {
        &TABLE_MapCompletionAchievements[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MapCompletionAchievements.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MapCompletionAchievementsRow)> {
        TABLE_MapCompletionAchievements.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_MapCompletionAchievements.iter() {
            black_box(row);
        }
    }
}
