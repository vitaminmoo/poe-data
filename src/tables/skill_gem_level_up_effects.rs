#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_SkillGemLevelUpEffects: LazyLock<Vec<SkillGemLevelUpEffectsRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/skillgemlevelupeffects.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| SkillGemLevelUpEffectsRow {
                r#id: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(0..0 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#unknown8: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(8..8 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    value
                },
                r#unknown24: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(24..24 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    value
                },
                r#unknown40: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(40..40 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    value
                },
                r#unknown56: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(56..56 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct SkillGemLevelUpEffectsRow {
    pub r#id: String,
    pub r#unknown8: i64,
    pub r#unknown24: i64,
    pub r#unknown40: i64,
    pub r#unknown56: i64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct SkillGemLevelUpEffectsRef(pub usize);

impl Deref for SkillGemLevelUpEffectsRef {
    type Target = SkillGemLevelUpEffectsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_SkillGemLevelUpEffects[self.0]
    }
}

impl SkillGemLevelUpEffectsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static SkillGemLevelUpEffectsRow {
        &TABLE_SkillGemLevelUpEffects[self.0]
    }
    pub fn get(&self) -> &'static SkillGemLevelUpEffectsRow {
        &TABLE_SkillGemLevelUpEffects[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_SkillGemLevelUpEffects
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static SkillGemLevelUpEffectsRow)> {
        TABLE_SkillGemLevelUpEffects
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
        for row in TABLE_SkillGemLevelUpEffects.iter() {
            black_box(row);
        }
    }
}
