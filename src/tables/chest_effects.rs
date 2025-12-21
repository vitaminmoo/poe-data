#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ChestEffects: LazyLock<Vec<ChestEffectsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/chesteffects.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ChestEffectsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#normal_epk_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#normal_closed_ao_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#normal_open_ao_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(24..24 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#magic_epk_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(32..32 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unique_epk_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(40..40 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#rare_epk_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(48..48 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#magic_closed_ao_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(56..56 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unique_closed_ao_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(64..64 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#rare_closed_ao_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(72..72 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#magic_open_ao_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(80..80 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unique_open_ao_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(88..88 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#rare_open_ao_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(96..96 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ChestEffectsRow {
    pub r#id: String,
    pub r#normal_epk_file: String,
    pub r#normal_closed_ao_file: String,
    pub r#normal_open_ao_file: String,
    pub r#magic_epk_file: String,
    pub r#unique_epk_file: String,
    pub r#rare_epk_file: String,
    pub r#magic_closed_ao_file: String,
    pub r#unique_closed_ao_file: String,
    pub r#rare_closed_ao_file: String,
    pub r#magic_open_ao_file: String,
    pub r#unique_open_ao_file: String,
    pub r#rare_open_ao_file: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ChestEffectsRef(pub usize);

impl Deref for ChestEffectsRef {
    type Target = ChestEffectsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ChestEffects[self.0]
    }
}

impl ChestEffectsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ChestEffectsRow {
        &TABLE_ChestEffects[self.0]
    }
    pub fn get(&self) -> &'static ChestEffectsRow {
        &TABLE_ChestEffects[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ChestEffects.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ChestEffectsRow)> {
        TABLE_ChestEffects
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
        for row in TABLE_ChestEffects.iter() {
            black_box(row);
        }
    }
}
