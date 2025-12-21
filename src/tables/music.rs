#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_Music: LazyLock<Vec<MusicRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/music.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| MusicRow {
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
            r#bank_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#hash16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#is_available_in_hideout: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(28).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(29..29 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown37: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(37..37 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#music_categories: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(45..45 + 16).unwrap();
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
                    .into_iter()
                    .map(|value| MusicCategoriesRef::new(value as usize))
                    .collect()
            },
            r#unknown61: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(61).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown62: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(62..62 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MusicRow {
    pub r#id: String,
    pub r#sound_file: String,
    pub r#bank_file: String,
    pub r#hash16: i32,
    pub r#is_available_in_hideout: bool,
    pub r#name: String,
    pub r#unknown37: String,
    pub r#music_categories: Vec<MusicCategoriesRef>,
    pub r#unknown61: bool,
    pub r#unknown62: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MusicRef(pub usize);

impl Deref for MusicRef {
    type Target = MusicRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_Music[self.0]
    }
}

impl MusicRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MusicRow {
        &TABLE_Music[self.0]
    }
    pub fn get(&self) -> &'static MusicRow {
        &TABLE_Music[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Music.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MusicRow)> {
        TABLE_Music.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_Music.iter() {
            black_box(row);
        }
    }
}
