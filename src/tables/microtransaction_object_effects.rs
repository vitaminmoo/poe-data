#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MicrotransactionObjectEffects: LazyLock<Vec<MicrotransactionObjectEffectsRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/microtransactionobjecteffects.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| MicrotransactionObjectEffectsRow {
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
                r#script: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(24..24 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#unknown32: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(32..32 + 16).unwrap();
                    let count = cell_bytes.get_u64_le() as usize;
                    let offset = cell_bytes.get_u64_le() as usize;
                    // array_mutator column.array == true && column.type == 'array'
                    let values = (count, offset);
                    values
                },
                r#unknown48: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(48..48 + 16).unwrap();
                    let count = cell_bytes.get_u64_le() as usize;
                    let offset = cell_bytes.get_u64_le() as usize;
                    // array_mutator column.array == true && column.type == 'array'
                    let values = (count, offset);
                    values
                },
                r#unknown64: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(64..64 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    value
                },
                r#unknown80: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(80..80 + 16).unwrap();
                    let count = cell_bytes.get_u64_le() as usize;
                    let offset = cell_bytes.get_u64_le() as usize;
                    // array_mutator column.array == true && column.type == 'array'
                    let values = (count, offset);
                    values
                },
                r#unknown96: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(96..96 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown100: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(100..100 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown104: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(104..104 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown108: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(108..108 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown112: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(112..112 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown116: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(116..116 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown120: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(120..120 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown124: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(124..124 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown128: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(128..128 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown132: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(132..132 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct MicrotransactionObjectEffectsRow {
    pub r#id: String,
    pub r#unknown8: (usize, usize),
    pub r#script: String,
    pub r#unknown32: (usize, usize),
    pub r#unknown48: (usize, usize),
    pub r#unknown64: i64,
    pub r#unknown80: (usize, usize),
    pub r#unknown96: i32,
    pub r#unknown100: i32,
    pub r#unknown104: i32,
    pub r#unknown108: i32,
    pub r#unknown112: i32,
    pub r#unknown116: i32,
    pub r#unknown120: i32,
    pub r#unknown124: i32,
    pub r#unknown128: i32,
    pub r#unknown132: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MicrotransactionObjectEffectsRef(pub usize);

impl Deref for MicrotransactionObjectEffectsRef {
    type Target = MicrotransactionObjectEffectsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MicrotransactionObjectEffects[self.0]
    }
}

impl MicrotransactionObjectEffectsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MicrotransactionObjectEffectsRow {
        &TABLE_MicrotransactionObjectEffects[self.0]
    }
    pub fn get(&self) -> &'static MicrotransactionObjectEffectsRow {
        &TABLE_MicrotransactionObjectEffects[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MicrotransactionObjectEffects
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs(
    ) -> impl Iterator<Item = (Self, &'static MicrotransactionObjectEffectsRow)> {
        TABLE_MicrotransactionObjectEffects
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
        for row in TABLE_MicrotransactionObjectEffects.iter() {
            black_box(row);
        }
    }
}
