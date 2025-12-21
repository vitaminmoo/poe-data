#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_Stats: LazyLock<Vec<StatsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/stats.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| StatsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown8: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(8).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#is_local: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(9).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#is_weapon_local: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(10).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#semantic: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(11..11 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                StatSemantics::from_repr(value as usize)
            },
            r#unknown15: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(15).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#is_virtual: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(16).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#main_hand_alias_stat: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(17..17 + 8).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#off_hand_alias_stat: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(25..25 + 8).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#unknown33: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(33).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#hash32: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(34..34 + 4).unwrap();
                let value = cell_bytes.get_u32_le();
                value
            },
            r#belongs_active_skills: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(38..38 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#category: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(54..54 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                PassiveSkillStatCategoriesRef::new(value as usize)
            },
            r#unknown70: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(70).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown71: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(71).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#is_scalable: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(72).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#context_flags: {
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
                values
                    .into_iter()
                    .map(|value| VirtualStatContextFlagsRef::new(value as usize))
                    .collect()
            },
            r#dot_flag: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(89..89 + 16).unwrap();
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
                    .map(|value| VirtualStatContextFlagsRef::new(value as usize))
                    .collect()
            },
            r#weapon_hand_check: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(105).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct StatsRow {
    pub r#id: String,
    pub r#unknown8: bool,
    pub r#is_local: bool,
    pub r#is_weapon_local: bool,
    pub r#semantic: Option<StatSemantics>,
    pub r#unknown15: bool,
    pub r#is_virtual: bool,
    pub r#main_hand_alias_stat: StatsRef,
    pub r#off_hand_alias_stat: StatsRef,
    pub r#unknown33: bool,
    pub r#hash32: u32,
    pub r#belongs_active_skills: Vec<String>,
    pub r#category: PassiveSkillStatCategoriesRef,
    pub r#unknown70: bool,
    pub r#unknown71: bool,
    pub r#is_scalable: bool,
    pub r#context_flags: Vec<VirtualStatContextFlagsRef>,
    pub r#dot_flag: Vec<VirtualStatContextFlagsRef>,
    pub r#weapon_hand_check: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct StatsRef(pub usize);

impl Deref for StatsRef {
    type Target = StatsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_Stats[self.0]
    }
}

impl StatsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static StatsRow {
        &TABLE_Stats[self.0]
    }
    pub fn get(&self) -> &'static StatsRow {
        &TABLE_Stats[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Stats.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static StatsRow)> {
        TABLE_Stats.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn print_all_rows() {
        // Print all rows
        for row in TABLE_Stats.iter() {
            println!("{:?}", row);
        }
    }
}
