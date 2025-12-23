#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MonsterConditions: LazyLock<Vec<MonsterConditionsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/monsterconditions.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| MonsterConditionsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#rarity: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                RarityRef::new(value as usize)
            },
            r#stat: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#not_rarity: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                RarityRef::new(value as usize)
            },
            r#not_stat: {
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
                values.into_iter().map(|value| StatsRef::new(value as usize)).collect()
            },
            r#map_boss: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(72).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#not_map_boss: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(73).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown74: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(74..74 + 16).unwrap();
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
            },
            r#unknown90: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(90..90 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown106: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(106..106 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown110: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(110..110 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown114: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(114..114 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#hash32: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(118..118 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MonsterConditionsRow {
    pub r#id: String,
    pub r#rarity: RarityRef,
    pub r#stat: StatsRef,
    pub r#not_rarity: RarityRef,
    pub r#not_stat: Vec<StatsRef>,
    pub r#map_boss: bool,
    pub r#not_map_boss: bool,
    pub r#unknown74: Vec<i64>,
    pub r#unknown90: Vec<String>,
    pub r#unknown106: i32,
    pub r#unknown110: i32,
    pub r#unknown114: i32,
    pub r#hash32: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MonsterConditionsRef(pub usize);

impl Deref for MonsterConditionsRef {
    type Target = MonsterConditionsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MonsterConditions[self.0]
    }
}

impl MonsterConditionsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MonsterConditionsRow {
        &TABLE_MonsterConditions[self.0]
    }
    pub fn get(&self) -> &'static MonsterConditionsRow {
        &TABLE_MonsterConditions[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MonsterConditions.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MonsterConditionsRow)> {
        TABLE_MonsterConditions.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_MonsterConditions.iter() {
            black_box(row);
        }
    }
}
