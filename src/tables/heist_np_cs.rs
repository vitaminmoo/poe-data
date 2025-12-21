#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_HeistNPCs: LazyLock<Vec<HeistNPCsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/heistnpcs.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| HeistNPCsRow {
            r#np_cs_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                NPCsRef::new(value as usize)
            },
            r#monster_varieties_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MonsterVarietiesRef::new(value as usize)
            },
            r#skill_level_heist_jobs_keys: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
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
                    .map(|value| HeistJobsRef::new(value as usize))
                    .collect()
            },
            r#portrait_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(48..48 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#heist_npc_stats_keys: {
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
                values
                    .into_iter()
                    .map(|value| HeistNPCStatsRef::new(value as usize))
                    .collect()
            },
            r#stat_values: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(72..72 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 4)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_f32_le())
                    .collect::<Vec<f32>>();
                values
            },
            r#unknown88: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(88..88 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#skill_level_values: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(92..92 + 16).unwrap();
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
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(108..108 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#silhouette_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(116..116 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown124: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(124..124 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown128: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(128..128 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#heist_np_cs_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(132..132 + 8).unwrap();
                let value = cell_bytes.get_i64_le();
                HeistNPCsRef::new(value as usize)
            },
            r#stat_values2: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(140..140 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 4)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_f32_le())
                    .collect::<Vec<f32>>();
                values
            },
            r#ally_np_cs_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(156..156 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                NPCsRef::new(value as usize)
            },
            r#active_npc_icon: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(172..172 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#heist_jobs_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(180..180 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                HeistJobsRef::new(value as usize)
            },
            r#equip_achievement_items_keys: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(196..196 + 16).unwrap();
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
            r#ao_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(212..212 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown220: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(220..220 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct HeistNPCsRow {
    pub r#np_cs_key: NPCsRef,
    pub r#monster_varieties_key: MonsterVarietiesRef,
    pub r#skill_level_heist_jobs_keys: Vec<HeistJobsRef>,
    pub r#portrait_file: String,
    pub r#heist_npc_stats_keys: Vec<HeistNPCStatsRef>,
    pub r#stat_values: Vec<f32>,
    pub r#unknown88: f32,
    pub r#skill_level_values: Vec<i32>,
    pub r#name: String,
    pub r#silhouette_file: String,
    pub r#unknown124: i32,
    pub r#unknown128: i32,
    pub r#heist_np_cs_key: HeistNPCsRef,
    pub r#stat_values2: Vec<f32>,
    pub r#ally_np_cs_key: NPCsRef,
    pub r#active_npc_icon: String,
    pub r#heist_jobs_key: HeistJobsRef,
    pub r#equip_achievement_items_keys: Vec<AchievementItemsRef>,
    pub r#ao_file: String,
    pub r#unknown220: i64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct HeistNPCsRef(pub usize);

impl Deref for HeistNPCsRef {
    type Target = HeistNPCsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_HeistNPCs[self.0]
    }
}

impl HeistNPCsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static HeistNPCsRow {
        &TABLE_HeistNPCs[self.0]
    }
    pub fn get(&self) -> &'static HeistNPCsRow {
        &TABLE_HeistNPCs[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_HeistNPCs.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static HeistNPCsRow)> {
        TABLE_HeistNPCs
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
        for row in TABLE_HeistNPCs.iter() {
            black_box(row);
        }
    }
}
