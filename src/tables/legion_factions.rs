#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_LegionFactions: LazyLock<Vec<LegionFactionsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/legionfactions.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| LegionFactionsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#spawn_weight: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#legion_balance_per_level_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(12..12 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown28: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(28..28 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown32: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#buff_visuals_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(36..36 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BuffVisualsRef::new(value as usize)
            },
            r#misc_animated_key1: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(52..52 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MiscAnimatedRef::new(value as usize)
            },
            r#misc_animated_key2: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(68..68 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MiscAnimatedRef::new(value as usize)
            },
            r#misc_animated_key3: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(84..84 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MiscAnimatedRef::new(value as usize)
            },
            r#achievement_items_keys1: {
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
                    .map(|value| AchievementItemsRef::new(value as usize))
                    .collect()
            },
            r#misc_animated_key4: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(116..116 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MiscAnimatedRef::new(value as usize)
            },
            r#misc_animated_key5: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(132..132 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MiscAnimatedRef::new(value as usize)
            },
            r#unknown148: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(148..148 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown152: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(152..152 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#achievement_items_keys2: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(156..156 + 16).unwrap();
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
            r#stats_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(172..172 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#shard: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(188..188 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#reward_jewel_art: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(196..196 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct LegionFactionsRow {
    pub r#id: String,
    pub r#spawn_weight: i32,
    pub r#legion_balance_per_level_key: i64,
    pub r#unknown28: f32,
    pub r#unknown32: f32,
    pub r#buff_visuals_key: BuffVisualsRef,
    pub r#misc_animated_key1: MiscAnimatedRef,
    pub r#misc_animated_key2: MiscAnimatedRef,
    pub r#misc_animated_key3: MiscAnimatedRef,
    pub r#achievement_items_keys1: Vec<AchievementItemsRef>,
    pub r#misc_animated_key4: MiscAnimatedRef,
    pub r#misc_animated_key5: MiscAnimatedRef,
    pub r#unknown148: f32,
    pub r#unknown152: f32,
    pub r#achievement_items_keys2: Vec<AchievementItemsRef>,
    pub r#stats_key: StatsRef,
    pub r#shard: String,
    pub r#reward_jewel_art: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct LegionFactionsRef(pub usize);

impl Deref for LegionFactionsRef {
    type Target = LegionFactionsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_LegionFactions[self.0]
    }
}

impl LegionFactionsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static LegionFactionsRow {
        &TABLE_LegionFactions[self.0]
    }
    pub fn get(&self) -> &'static LegionFactionsRow {
        &TABLE_LegionFactions[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_LegionFactions
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static LegionFactionsRow)> {
        TABLE_LegionFactions
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
        for row in TABLE_LegionFactions.iter() {
            black_box(row);
        }
    }
}
