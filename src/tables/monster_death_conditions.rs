#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MonsterDeathConditions: LazyLock<Vec<MonsterDeathConditionsRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/monsterdeathconditions.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| MonsterDeathConditionsRow {
                r#unknown0: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(0..0 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#unknown8: {
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
                    values
                },
                r#unknown24: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(24).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#unknown25: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(25..25 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown29: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(29..29 + 16).unwrap();
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
                r#unknown45: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(45).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#unknown46: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(46..46 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown50: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(50..50 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    value
                },
                r#unknown66: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(66).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#unknown67: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(67..67 + 16).unwrap();
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
                r#unknown83: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(83..83 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown87: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(87).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#unknown88: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(88..88 + 16).unwrap();
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
                r#unknown104: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(104..104 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown108: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(108..108 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    value
                },
                r#unknown124: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(124..124 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    value
                },
                r#unknown140: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(140..140 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown144: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(144..144 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct MonsterDeathConditionsRow {
    pub r#unknown0: String,
    pub r#unknown8: Vec<i64>,
    pub r#unknown24: bool,
    pub r#unknown25: i32,
    pub r#unknown29: Vec<i64>,
    pub r#unknown45: bool,
    pub r#unknown46: i32,
    pub r#unknown50: i64,
    pub r#unknown66: bool,
    pub r#unknown67: Vec<i64>,
    pub r#unknown83: i32,
    pub r#unknown87: bool,
    pub r#unknown88: Vec<i64>,
    pub r#unknown104: i32,
    pub r#unknown108: i64,
    pub r#unknown124: i64,
    pub r#unknown140: i32,
    pub r#unknown144: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MonsterDeathConditionsRef(pub usize);

impl Deref for MonsterDeathConditionsRef {
    type Target = MonsterDeathConditionsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MonsterDeathConditions[self.0]
    }
}

impl MonsterDeathConditionsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MonsterDeathConditionsRow {
        &TABLE_MonsterDeathConditions[self.0]
    }
    pub fn get(&self) -> &'static MonsterDeathConditionsRow {
        &TABLE_MonsterDeathConditions[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MonsterDeathConditions
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MonsterDeathConditionsRow)> {
        TABLE_MonsterDeathConditions
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
        for row in TABLE_MonsterDeathConditions.iter() {
            black_box(row);
        }
    }
}
