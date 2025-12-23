#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_Shrines: LazyLock<Vec<ShrinesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/shrines.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| ShrinesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#timeout_in_seconds: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#charges_shared: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(12).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#player_shrine_buffs_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(13..13 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown29: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(29..29 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown33: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(33..33 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#monster_shrine_buffs_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(37..37 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#summon_monster_monster_varieties_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(53..53 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MonsterVarietiesRef::new(value as usize)
            },
            r#summon_player_monster_varieties_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(69..69 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MonsterVarietiesRef::new(value as usize)
            },
            r#unknown85: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(85..85 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#duration: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(89..89 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#shrine_sounds_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(93..93 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ShrineSoundsRef::new(value as usize)
            },
            r#unknown109: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(109).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#achievement_items_keys: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(110..110 + 16).unwrap();
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
            r#is_pvp_only: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(126).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown127: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(127).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#is_lesser_shrine: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(128).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#description: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(129..129 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ClientStringsRef::new(value as usize)
            },
            r#name: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(145..145 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ClientStringsRef::new(value as usize)
            },
            r#unknown161: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(161).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown162: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(162..162 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown178: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(178..178 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown194: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(194..194 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown210: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(210..210 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#unknown226: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(226..226 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ShrinesRow {
    pub r#id: String,
    pub r#timeout_in_seconds: i32,
    pub r#charges_shared: bool,
    pub r#player_shrine_buffs_key: i64,
    pub r#unknown29: i32,
    pub r#unknown33: i32,
    pub r#monster_shrine_buffs_key: i64,
    pub r#summon_monster_monster_varieties_key: MonsterVarietiesRef,
    pub r#summon_player_monster_varieties_key: MonsterVarietiesRef,
    pub r#unknown85: i32,
    pub r#duration: i32,
    pub r#shrine_sounds_key: ShrineSoundsRef,
    pub r#unknown109: bool,
    pub r#achievement_items_keys: Vec<AchievementItemsRef>,
    pub r#is_pvp_only: bool,
    pub r#unknown127: bool,
    pub r#is_lesser_shrine: bool,
    pub r#description: ClientStringsRef,
    pub r#name: ClientStringsRef,
    pub r#unknown161: bool,
    pub r#unknown162: i64,
    pub r#unknown178: (usize, usize),
    pub r#unknown194: i64,
    pub r#unknown210: StatsRef,
    pub r#unknown226: StatsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ShrinesRef(pub usize);

impl Deref for ShrinesRef {
    type Target = ShrinesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_Shrines[self.0]
    }
}

impl ShrinesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ShrinesRow {
        &TABLE_Shrines[self.0]
    }
    pub fn get(&self) -> &'static ShrinesRow {
        &TABLE_Shrines[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Shrines.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ShrinesRow)> {
        TABLE_Shrines.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_Shrines.iter() {
            black_box(row);
        }
    }
}
