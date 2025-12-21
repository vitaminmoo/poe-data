#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_EffectDrivenSkill: LazyLock<Vec<EffectDrivenSkillRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/effectdrivenskill.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| EffectDrivenSkillRow {
            r#unknown0: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown4: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(4..4 + 16).unwrap();
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
            r#unknown20: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(20..20 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown36: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(36..36 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown40: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown44: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(44).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown45: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(45).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown46: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(46).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown47: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(47..47 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown63: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(63..63 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown67: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(67..67 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown71: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(71..71 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown75: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(75..75 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown79: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(79..79 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown83: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(83).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown84: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(84..84 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown88: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(88).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown89: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(89).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown90: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(90..90 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown94: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(94..94 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown98: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(98).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown99: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(99).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown100: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(100).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown101: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(101..101 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown105: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(105).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown106: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(106).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown107: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(107..107 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown111: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(111..111 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown115: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(115..115 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct EffectDrivenSkillRow {
    pub r#unknown0: i32,
    pub r#unknown4: Vec<i64>,
    pub r#unknown20: (usize, usize),
    pub r#unknown36: i32,
    pub r#unknown40: i32,
    pub r#unknown44: bool,
    pub r#unknown45: bool,
    pub r#unknown46: bool,
    pub r#unknown47: (usize, usize),
    pub r#unknown63: i32,
    pub r#unknown67: i32,
    pub r#unknown71: i32,
    pub r#unknown75: i32,
    pub r#unknown79: i32,
    pub r#unknown83: bool,
    pub r#unknown84: i32,
    pub r#unknown88: bool,
    pub r#unknown89: bool,
    pub r#unknown90: i32,
    pub r#unknown94: i32,
    pub r#unknown98: bool,
    pub r#unknown99: bool,
    pub r#unknown100: bool,
    pub r#unknown101: i32,
    pub r#unknown105: bool,
    pub r#unknown106: bool,
    pub r#unknown107: i32,
    pub r#unknown111: i32,
    pub r#unknown115: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct EffectDrivenSkillRef(pub usize);

impl Deref for EffectDrivenSkillRef {
    type Target = EffectDrivenSkillRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_EffectDrivenSkill[self.0]
    }
}

impl EffectDrivenSkillRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static EffectDrivenSkillRow {
        &TABLE_EffectDrivenSkill[self.0]
    }
    pub fn get(&self) -> &'static EffectDrivenSkillRow {
        &TABLE_EffectDrivenSkill[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_EffectDrivenSkill
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static EffectDrivenSkillRow)> {
        TABLE_EffectDrivenSkill
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
        for row in TABLE_EffectDrivenSkill.iter() {
            black_box(row);
        }
    }
}
