#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ShrineSounds: LazyLock<Vec<ShrineSoundsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/shrinesounds.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| ShrineSoundsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#stereo_sound_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#mono_sound_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ShrineSoundsRow {
    pub r#id: String,
    pub r#stereo_sound_file: String,
    pub r#mono_sound_file: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ShrineSoundsRef(pub usize);

impl Deref for ShrineSoundsRef {
    type Target = ShrineSoundsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ShrineSounds[self.0]
    }
}

impl ShrineSoundsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ShrineSoundsRow {
        &TABLE_ShrineSounds[self.0]
    }
    pub fn get(&self) -> &'static ShrineSoundsRow {
        &TABLE_ShrineSounds[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ShrineSounds.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ShrineSoundsRow)> {
        TABLE_ShrineSounds.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_ShrineSounds.iter() {
            black_box(row);
        }
    }
}
