#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MonsterConditionalEffectPacks: LazyLock<Vec<MonsterConditionalEffectPacksRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/monsterconditionaleffectpacks.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| MonsterConditionalEffectPacksRow {
                r#id: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(0..0 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#misc_effect_pack1: {
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
                        .map(|value| MiscEffectPacksRef::new(value as usize))
                        .collect()
                },
                r#misc_effect_pack2: {
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
                        .map(|value| MiscEffectPacksRef::new(value as usize))
                        .collect()
                },
                r#misc_effect_pack3: {
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
                        .map(|value| MiscEffectPacksRef::new(value as usize))
                        .collect()
                },
                r#misc_effect_pack4: {
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
                        .map(|value| MiscEffectPacksRef::new(value as usize))
                        .collect()
                },
                r#unknown72: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(72..72 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct MonsterConditionalEffectPacksRow {
    pub r#id: String,
    pub r#misc_effect_pack1: Vec<MiscEffectPacksRef>,
    pub r#misc_effect_pack2: Vec<MiscEffectPacksRef>,
    pub r#misc_effect_pack3: Vec<MiscEffectPacksRef>,
    pub r#misc_effect_pack4: Vec<MiscEffectPacksRef>,
    pub r#unknown72: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MonsterConditionalEffectPacksRef(pub usize);

impl Deref for MonsterConditionalEffectPacksRef {
    type Target = MonsterConditionalEffectPacksRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MonsterConditionalEffectPacks[self.0]
    }
}

impl MonsterConditionalEffectPacksRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MonsterConditionalEffectPacksRow {
        &TABLE_MonsterConditionalEffectPacks[self.0]
    }
    pub fn get(&self) -> &'static MonsterConditionalEffectPacksRow {
        &TABLE_MonsterConditionalEffectPacks[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MonsterConditionalEffectPacks
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs(
    ) -> impl Iterator<Item = (Self, &'static MonsterConditionalEffectPacksRow)> {
        TABLE_MonsterConditionalEffectPacks
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
        for row in TABLE_MonsterConditionalEffectPacks.iter() {
            black_box(row);
        }
    }
}
