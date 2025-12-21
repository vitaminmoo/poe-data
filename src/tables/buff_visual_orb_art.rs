#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_BuffVisualOrbArt: LazyLock<Vec<BuffVisualOrbArtRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/buffvisualorbart.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| BuffVisualOrbArtRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#misc_animated: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MiscAnimatedRef::new(value as usize)
            },
            r#unknown24: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
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
            r#unknown40: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(40..40 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown56: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(56..56 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown64: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(64..64 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown72: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(72..72 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown80: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(80..80 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown88: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(88..88 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown96: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(96..96 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown104: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(104).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown105: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(105..105 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct BuffVisualOrbArtRow {
    pub r#id: String,
    pub r#misc_animated: MiscAnimatedRef,
    pub r#unknown24: Vec<i32>,
    pub r#unknown40: Vec<String>,
    pub r#unknown56: String,
    pub r#unknown64: String,
    pub r#unknown72: String,
    pub r#unknown80: String,
    pub r#unknown88: String,
    pub r#unknown96: String,
    pub r#unknown104: bool,
    pub r#unknown105: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BuffVisualOrbArtRef(pub usize);

impl Deref for BuffVisualOrbArtRef {
    type Target = BuffVisualOrbArtRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_BuffVisualOrbArt[self.0]
    }
}

impl BuffVisualOrbArtRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BuffVisualOrbArtRow {
        &TABLE_BuffVisualOrbArt[self.0]
    }
    pub fn get(&self) -> &'static BuffVisualOrbArtRow {
        &TABLE_BuffVisualOrbArt[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BuffVisualOrbArt
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BuffVisualOrbArtRow)> {
        TABLE_BuffVisualOrbArt
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
        for row in TABLE_BuffVisualOrbArt.iter() {
            black_box(row);
        }
    }
}
