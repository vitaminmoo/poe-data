#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_DamageEffectVariations: LazyLock<Vec<DamageEffectVariationsRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/damageeffectvariations.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| DamageEffectVariationsRow {
                r#id: {
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
                    // array_mutator column.array == true && column.type == 'array'
                    let values = (count, offset);
                    values
                },
                r#unknown24: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(24..24 + 16).unwrap();
                    let count = cell_bytes.get_u64_le() as usize;
                    let offset = cell_bytes.get_u64_le() as usize;
                    // array_mutator column.array == true && column.type == 'string'
                    let values = df.strings_from_offset(offset, count).unwrap();
                    values
                },
                r#unknown40: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(40).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#unknown41: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(41..41 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown45: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(45..45 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown49: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(49..49 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown53: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(53..53 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown57: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(57..57 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown61: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(61..61 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown65: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(65..65 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown69: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(69..69 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown73: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(73..73 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown77: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(77).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#unknown78: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(78).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#unknown79: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(79).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#unknown80: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(80..80 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct DamageEffectVariationsRow {
    pub r#id: String,
    pub r#unknown8: (usize, usize),
    pub r#unknown24: Vec<String>,
    pub r#unknown40: bool,
    pub r#unknown41: i32,
    pub r#unknown45: i32,
    pub r#unknown49: i32,
    pub r#unknown53: i32,
    pub r#unknown57: i32,
    pub r#unknown61: i32,
    pub r#unknown65: i32,
    pub r#unknown69: i32,
    pub r#unknown73: i32,
    pub r#unknown77: bool,
    pub r#unknown78: bool,
    pub r#unknown79: bool,
    pub r#unknown80: i64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct DamageEffectVariationsRef(pub usize);

impl Deref for DamageEffectVariationsRef {
    type Target = DamageEffectVariationsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_DamageEffectVariations[self.0]
    }
}

impl DamageEffectVariationsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static DamageEffectVariationsRow {
        &TABLE_DamageEffectVariations[self.0]
    }
    pub fn get(&self) -> &'static DamageEffectVariationsRow {
        &TABLE_DamageEffectVariations[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_DamageEffectVariations
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static DamageEffectVariationsRow)> {
        TABLE_DamageEffectVariations
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
        for row in TABLE_DamageEffectVariations.iter() {
            black_box(row);
        }
    }
}
