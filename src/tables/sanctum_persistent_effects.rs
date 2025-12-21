#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_SanctumPersistentEffects: LazyLock<Vec<SanctumPersistentEffectsRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/sanctumpersistenteffects.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| SanctumPersistentEffectsRow {
                r#id: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(0..0 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#stats: {
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
                        .map(|value| StatsRef::new(value as usize))
                        .collect()
                },
                r#stat_values: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(24..24 + 16).unwrap();
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
                    let mut cell_bytes = row.get(40..40 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#icon: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(48..48 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#unknown56: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(56..56 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown60: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(60).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#effect_category: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(61..61 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    SanctumPersistentEffectCategoriesRef::new(value as usize)
                },
                r#next_effect: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(77..77 + 8).unwrap();
                    let value = cell_bytes.get_i64_le();
                    SanctumPersistentEffectsRef::new(value as usize)
                },
                r#unknown85: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(85..85 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#boon_desc: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(93..93 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#curse_desc: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(101..101 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#unknown109: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(109..109 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown113: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(113..113 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown117: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(117).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#unknown118: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(118..118 + 16).unwrap();
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
                r#guard: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(134..134 + 16).unwrap();
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
                r#first_effect: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(150..150 + 8).unwrap();
                    let value = cell_bytes.get_i64_le();
                    SanctumPersistentEffectsRef::new(value as usize)
                },
                r#unknown158: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(158..158 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown162: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(162..162 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown166: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(166).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#unknown167: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(167).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#hash16: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(168..168 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct SanctumPersistentEffectsRow {
    pub r#id: String,
    pub r#stats: Vec<StatsRef>,
    pub r#stat_values: Vec<i32>,
    pub r#name: String,
    pub r#icon: String,
    pub r#unknown56: i32,
    pub r#unknown60: bool,
    pub r#effect_category: SanctumPersistentEffectCategoriesRef,
    pub r#next_effect: SanctumPersistentEffectsRef,
    pub r#unknown85: String,
    pub r#boon_desc: String,
    pub r#curse_desc: String,
    pub r#unknown109: i32,
    pub r#unknown113: i32,
    pub r#unknown117: bool,
    pub r#unknown118: Vec<i64>,
    pub r#guard: Vec<MonsterVarietiesRef>,
    pub r#first_effect: SanctumPersistentEffectsRef,
    pub r#unknown158: i32,
    pub r#unknown162: i32,
    pub r#unknown166: bool,
    pub r#unknown167: bool,
    pub r#hash16: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct SanctumPersistentEffectsRef(pub usize);

impl Deref for SanctumPersistentEffectsRef {
    type Target = SanctumPersistentEffectsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_SanctumPersistentEffects[self.0]
    }
}

impl SanctumPersistentEffectsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static SanctumPersistentEffectsRow {
        &TABLE_SanctumPersistentEffects[self.0]
    }
    pub fn get(&self) -> &'static SanctumPersistentEffectsRow {
        &TABLE_SanctumPersistentEffects[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_SanctumPersistentEffects
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static SanctumPersistentEffectsRow)> {
        TABLE_SanctumPersistentEffects
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
        for row in TABLE_SanctumPersistentEffects.iter() {
            black_box(row);
        }
    }
}
