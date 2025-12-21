#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_SoundEffects: LazyLock<Vec<SoundEffectsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/soundeffects.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| SoundEffectsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#sound_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#sound_file_2_d: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown24: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(24).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown25: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(25..25 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct SoundEffectsRow {
    pub r#id: String,
    pub r#sound_file: String,
    pub r#sound_file_2_d: String,
    pub r#unknown24: bool,
    pub r#unknown25: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct SoundEffectsRef(pub usize);

impl Deref for SoundEffectsRef {
    type Target = SoundEffectsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_SoundEffects[self.0]
    }
}

impl SoundEffectsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static SoundEffectsRow {
        &TABLE_SoundEffects[self.0]
    }
    pub fn get(&self) -> &'static SoundEffectsRow {
        &TABLE_SoundEffects[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_SoundEffects.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static SoundEffectsRow)> {
        TABLE_SoundEffects
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
        for row in TABLE_SoundEffects.iter() {
            black_box(row);
        }
    }
}
