#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_PassiveSkillMasteryEffects: LazyLock<Vec<PassiveSkillMasteryEffectsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/passiveskillmasteryeffects.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| PassiveSkillMasteryEffectsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#hash16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 2).unwrap();
                let value = cell_bytes.get_u16_le();
                value
            },
            r#stats: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(10..10 + 16).unwrap();
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
            r#stat1_value: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(26..26 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#stat2_value: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(30..30 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#stat3_value: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(34..34 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#achievement_item: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(38..38 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                AchievementItemsRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct PassiveSkillMasteryEffectsRow {
    pub r#id: String,
    pub r#hash16: u16,
    pub r#stats: Vec<StatsRef>,
    pub r#stat1_value: i32,
    pub r#stat2_value: i32,
    pub r#stat3_value: i32,
    pub r#achievement_item: AchievementItemsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct PassiveSkillMasteryEffectsRef(pub usize);

impl Deref for PassiveSkillMasteryEffectsRef {
    type Target = PassiveSkillMasteryEffectsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_PassiveSkillMasteryEffects[self.0]
    }
}

impl PassiveSkillMasteryEffectsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static PassiveSkillMasteryEffectsRow {
        &TABLE_PassiveSkillMasteryEffects[self.0]
    }
    pub fn get(&self) -> &'static PassiveSkillMasteryEffectsRow {
        &TABLE_PassiveSkillMasteryEffects[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_PassiveSkillMasteryEffects.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static PassiveSkillMasteryEffectsRow)> {
        TABLE_PassiveSkillMasteryEffects.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_PassiveSkillMasteryEffects.iter() {
            black_box(row);
        }
    }
}
