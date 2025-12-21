#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_SkillSurgeEffects: LazyLock<Vec<SkillSurgeEffectsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/skillsurgeeffects.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| SkillSurgeEffectsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown8: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown16: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(16).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown17: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(17).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown18: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(18).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown19: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(19..19 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown35: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(35).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown36: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(36).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown37: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(37).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown38: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(38..38 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown42: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(42).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown43: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(43).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown44: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(44).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown45: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(45..45 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown61: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(61).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown62: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(62..62 + 16).unwrap();
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
            r#unknown78: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(78).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown79: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(79..79 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct SkillSurgeEffectsRow {
    pub r#id: String,
    pub r#unknown8: String,
    pub r#unknown16: bool,
    pub r#unknown17: bool,
    pub r#unknown18: bool,
    pub r#unknown19: i64,
    pub r#unknown35: bool,
    pub r#unknown36: bool,
    pub r#unknown37: bool,
    pub r#unknown38: i32,
    pub r#unknown42: bool,
    pub r#unknown43: bool,
    pub r#unknown44: bool,
    pub r#unknown45: i64,
    pub r#unknown61: bool,
    pub r#unknown62: Vec<i64>,
    pub r#unknown78: bool,
    pub r#unknown79: f32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct SkillSurgeEffectsRef(pub usize);

impl Deref for SkillSurgeEffectsRef {
    type Target = SkillSurgeEffectsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_SkillSurgeEffects[self.0]
    }
}

impl SkillSurgeEffectsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static SkillSurgeEffectsRow {
        &TABLE_SkillSurgeEffects[self.0]
    }
    pub fn get(&self) -> &'static SkillSurgeEffectsRow {
        &TABLE_SkillSurgeEffects[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_SkillSurgeEffects
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static SkillSurgeEffectsRow)> {
        TABLE_SkillSurgeEffects
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
        for row in TABLE_SkillSurgeEffects.iter() {
            black_box(row);
        }
    }
}
