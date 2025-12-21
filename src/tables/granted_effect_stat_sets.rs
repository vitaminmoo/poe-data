#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_GrantedEffectStatSets: LazyLock<Vec<GrantedEffectStatSetsRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/grantedeffectstatsets.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| GrantedEffectStatSetsRow {
                r#id: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(0..0 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#label: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(8..8 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    GrantedEffectLabelsRef::new(value as usize)
                },
                r#implicit_stats: {
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
                    values
                        .into_iter()
                        .map(|value| StatsRef::new(value as usize))
                        .collect()
                },
                r#constant_stats: {
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
                    values
                        .into_iter()
                        .map(|value| StatsRef::new(value as usize))
                        .collect()
                },
                r#constant_stats_values: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(56..56 + 16).unwrap();
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
                r#base_effectiveness: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(72..72 + 4).unwrap();
                    let value = cell_bytes.get_f32_le();
                    value
                },
                r#incremental_effectiveness: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(76..76 + 4).unwrap();
                    let value = cell_bytes.get_f32_le();
                    value
                },
                r#damage_incremental_effectiveness: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(80..80 + 4).unwrap();
                    let value = cell_bytes.get_f32_le();
                    value
                },
                r#ignored_stats: {
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
                        .map(|value| StatsRef::new(value as usize))
                        .collect()
                },
                r#unknown100: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(100).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct GrantedEffectStatSetsRow {
    pub r#id: String,
    pub r#label: GrantedEffectLabelsRef,
    pub r#implicit_stats: Vec<StatsRef>,
    pub r#constant_stats: Vec<StatsRef>,
    pub r#constant_stats_values: Vec<i32>,
    pub r#base_effectiveness: f32,
    pub r#incremental_effectiveness: f32,
    pub r#damage_incremental_effectiveness: f32,
    pub r#ignored_stats: Vec<StatsRef>,
    pub r#unknown100: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct GrantedEffectStatSetsRef(pub usize);

impl Deref for GrantedEffectStatSetsRef {
    type Target = GrantedEffectStatSetsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_GrantedEffectStatSets[self.0]
    }
}

impl GrantedEffectStatSetsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static GrantedEffectStatSetsRow {
        &TABLE_GrantedEffectStatSets[self.0]
    }
    pub fn get(&self) -> &'static GrantedEffectStatSetsRow {
        &TABLE_GrantedEffectStatSets[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_GrantedEffectStatSets
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static GrantedEffectStatSetsRow)> {
        TABLE_GrantedEffectStatSets
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
        for row in TABLE_GrantedEffectStatSets.iter() {
            black_box(row);
        }
    }
}
