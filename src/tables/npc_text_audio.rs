#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_NPCTextAudio: LazyLock<Vec<NPCTextAudioRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/npctextaudio.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| NPCTextAudioRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#characters: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| CharactersRef::new(value as usize)).collect()
            },
            r#text: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(24..24 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#audio_files: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown48: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(48..48 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown52: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(52..52 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown56: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(56).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown57: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(57).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown58: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(58..58 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown62: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(62..62 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown66: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(66..66 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#np_cs: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(70..70 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| NPCsRef::new(value as usize)).collect()
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct NPCTextAudioRow {
    pub r#id: String,
    pub r#characters: Vec<CharactersRef>,
    pub r#text: String,
    pub r#audio_files: Vec<String>,
    pub r#unknown48: i32,
    pub r#unknown52: i32,
    pub r#unknown56: bool,
    pub r#unknown57: bool,
    pub r#unknown58: i32,
    pub r#unknown62: i32,
    pub r#unknown66: i32,
    pub r#np_cs: Vec<NPCsRef>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct NPCTextAudioRef(pub usize);

impl Deref for NPCTextAudioRef {
    type Target = NPCTextAudioRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_NPCTextAudio[self.0]
    }
}

impl NPCTextAudioRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static NPCTextAudioRow {
        &TABLE_NPCTextAudio[self.0]
    }
    pub fn get(&self) -> &'static NPCTextAudioRow {
        &TABLE_NPCTextAudio[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_NPCTextAudio.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static NPCTextAudioRow)> {
        TABLE_NPCTextAudio.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_NPCTextAudio.iter() {
            black_box(row);
        }
    }
}
