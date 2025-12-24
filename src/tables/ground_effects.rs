#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_GroundEffects: LazyLock<Vec<GroundEffectsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/groundeffects.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| GroundEffectsRow {
            r#ground_effect_types_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                GroundEffectTypesRef::new(value as usize)
            },
            r#unknown16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown20: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
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
                let mut cell_bytes = row.get(40..40 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown44: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(44..44 + 16).unwrap();
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
            r#unknown60: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(60).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#ao_file: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(61..61 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown77: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(77..77 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#end_effect: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(93..93 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown101: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(101..101 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown117: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(117..117 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown133: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(133..133 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown149: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(149..149 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown165: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(165..165 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown181: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(181).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown182: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(182).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown183: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(183).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown184: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(184..184 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown200: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(200..200 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown216: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(216).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown217: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(217).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown218: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(218).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct GroundEffectsRow {
    pub r#ground_effect_types_key: GroundEffectTypesRef,
    pub r#unknown16: i32,
    pub r#unknown20: i32,
    pub r#unknown24: i64,
    pub r#unknown40: f32,
    pub r#unknown44: Vec<i32>,
    pub r#unknown60: bool,
    pub r#ao_file: Vec<String>,
    pub r#unknown77: Vec<String>,
    pub r#end_effect: String,
    pub r#unknown101: i64,
    pub r#unknown117: i64,
    pub r#unknown133: i64,
    pub r#unknown149: i64,
    pub r#unknown165: i64,
    pub r#unknown181: bool,
    pub r#unknown182: bool,
    pub r#unknown183: bool,
    pub r#unknown184: i64,
    pub r#unknown200: i64,
    pub r#unknown216: bool,
    pub r#unknown217: bool,
    pub r#unknown218: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct GroundEffectsRef(pub usize);

impl Deref for GroundEffectsRef {
    type Target = GroundEffectsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_GroundEffects[self.0]
    }
}

impl GroundEffectsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static GroundEffectsRow {
        &TABLE_GroundEffects[self.0]
    }
    pub fn get(&self) -> &'static GroundEffectsRow {
        &TABLE_GroundEffects[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_GroundEffects.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static GroundEffectsRow)> {
        TABLE_GroundEffects.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_GroundEffects.iter() {
            black_box(row);
        }
    }
}
