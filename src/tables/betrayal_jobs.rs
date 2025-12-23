#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_BetrayalJobs: LazyLock<Vec<BetrayalJobsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/betrayaljobs.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| BetrayalJobsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#text: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#extra_terrain_features_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ExtraTerrainFeaturesRef::new(value as usize)
            },
            r#art: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(32..32 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown40: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown44: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(44..44 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#world_areas_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(48..48 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                WorldAreasRef::new(value as usize)
            },
            r#completion_achievement_items_key: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(64..64 + 16).unwrap();
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
            r#open_chests_achievement_items_key: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(80..80 + 16).unwrap();
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
            r#mission_completion_acheivement_items_key: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(96..96 + 16).unwrap();
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
        })
        .collect()
});

#[derive(Debug)]
pub struct BetrayalJobsRow {
    pub r#id: String,
    pub r#text: String,
    pub r#extra_terrain_features_key: ExtraTerrainFeaturesRef,
    pub r#art: String,
    pub r#unknown40: i32,
    pub r#unknown44: i32,
    pub r#world_areas_key: WorldAreasRef,
    pub r#completion_achievement_items_key: Vec<AchievementItemsRef>,
    pub r#open_chests_achievement_items_key: Vec<AchievementItemsRef>,
    pub r#mission_completion_acheivement_items_key: Vec<AchievementItemsRef>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BetrayalJobsRef(pub usize);

impl Deref for BetrayalJobsRef {
    type Target = BetrayalJobsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_BetrayalJobs[self.0]
    }
}

impl BetrayalJobsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BetrayalJobsRow {
        &TABLE_BetrayalJobs[self.0]
    }
    pub fn get(&self) -> &'static BetrayalJobsRow {
        &TABLE_BetrayalJobs[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BetrayalJobs.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BetrayalJobsRow)> {
        TABLE_BetrayalJobs.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_BetrayalJobs.iter() {
            black_box(row);
        }
    }
}
