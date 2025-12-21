#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_NearbyMonsterConditions: LazyLock<Vec<NearbyMonsterConditionsRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/nearbymonsterconditions.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| NearbyMonsterConditionsRow {
                r#id: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(0..0 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#monster_varieties_keys: {
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
                        .into_iter()
                        .map(|value| MonsterVarietiesRef::new(value as usize))
                        .collect()
                },
                r#monster_amount: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(24..24 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown28: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(28..28 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#is_negated: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(32).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#unknown33: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(33..33 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown37: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(37..37 + 16).unwrap();
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
                r#is_less_then: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(53).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#minimum_health_percentage: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(54..54 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct NearbyMonsterConditionsRow {
    pub r#id: String,
    pub r#monster_varieties_keys: Vec<MonsterVarietiesRef>,
    pub r#monster_amount: i32,
    pub r#unknown28: i32,
    pub r#is_negated: bool,
    pub r#unknown33: i32,
    pub r#unknown37: Vec<i32>,
    pub r#is_less_then: bool,
    pub r#minimum_health_percentage: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct NearbyMonsterConditionsRef(pub usize);

impl Deref for NearbyMonsterConditionsRef {
    type Target = NearbyMonsterConditionsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_NearbyMonsterConditions[self.0]
    }
}

impl NearbyMonsterConditionsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static NearbyMonsterConditionsRow {
        &TABLE_NearbyMonsterConditions[self.0]
    }
    pub fn get(&self) -> &'static NearbyMonsterConditionsRow {
        &TABLE_NearbyMonsterConditions[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_NearbyMonsterConditions
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static NearbyMonsterConditionsRow)> {
        TABLE_NearbyMonsterConditions
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
        for row in TABLE_NearbyMonsterConditions.iter() {
            black_box(row);
        }
    }
}
