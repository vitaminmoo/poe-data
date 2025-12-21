#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_RitualRuneTypes: LazyLock<Vec<RitualRuneTypesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/ritualrunetypes.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| RitualRuneTypesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#misc_animated_key1: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MiscAnimatedRef::new(value as usize)
            },
            r#spawn_weight: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#level_min: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(28..28 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#level_max: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#buff_definitions_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(36..36 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BuffDefinitionsRef::new(value as usize)
            },
            r#buff_stat_values: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(52..52 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 4)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i32_le())
                    .collect::<Vec<i32>>();
                values
            },
            r#spawn_patterns: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(68..68 + 16).unwrap();
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
                    .map(|value| RitualSpawnPatternsRef::new(value as usize))
                    .collect()
            },
            r#mods_key: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(84..84 + 16).unwrap();
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
                    .map(|value| ModsRef::new(value as usize))
                    .collect()
            },
            r#unknown100: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(100..100 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 4)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i32_le())
                    .collect::<Vec<i32>>();
                values
            },
            r#unknown116: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(116..116 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 4)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i32_le())
                    .collect::<Vec<i32>>();
                values
            },
            r#misc_animated_key2: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(132..132 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MiscAnimatedRef::new(value as usize)
            },
            r#environments_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(148..148 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                EnvironmentsRef::new(value as usize)
            },
            r#unknown164: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(164..164 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#achievements: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(168..168 + 16).unwrap();
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
                    .map(|value| AchievementItemsRef::new(value as usize))
                    .collect()
            },
            r#type: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(184..184 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#description: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(192..192 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#daemon_spawning_data_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(200..200 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                DaemonSpawningDataRef::new(value as usize)
            },
            r#unknown216: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(216).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct RitualRuneTypesRow {
    pub r#id: String,
    pub r#misc_animated_key1: MiscAnimatedRef,
    pub r#spawn_weight: i32,
    pub r#level_min: i32,
    pub r#level_max: i32,
    pub r#buff_definitions_key: BuffDefinitionsRef,
    pub r#buff_stat_values: Vec<i32>,
    pub r#spawn_patterns: Vec<RitualSpawnPatternsRef>,
    pub r#mods_key: Vec<ModsRef>,
    pub r#unknown100: Vec<i32>,
    pub r#unknown116: Vec<i32>,
    pub r#misc_animated_key2: MiscAnimatedRef,
    pub r#environments_key: EnvironmentsRef,
    pub r#unknown164: i32,
    pub r#achievements: Vec<AchievementItemsRef>,
    pub r#type: String,
    pub r#description: String,
    pub r#daemon_spawning_data_key: DaemonSpawningDataRef,
    pub r#unknown216: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct RitualRuneTypesRef(pub usize);

impl Deref for RitualRuneTypesRef {
    type Target = RitualRuneTypesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_RitualRuneTypes[self.0]
    }
}

impl RitualRuneTypesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static RitualRuneTypesRow {
        &TABLE_RitualRuneTypes[self.0]
    }
    pub fn get(&self) -> &'static RitualRuneTypesRow {
        &TABLE_RitualRuneTypes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_RitualRuneTypes
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static RitualRuneTypesRow)> {
        TABLE_RitualRuneTypes
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
        for row in TABLE_RitualRuneTypes.iter() {
            black_box(row);
        }
    }
}
