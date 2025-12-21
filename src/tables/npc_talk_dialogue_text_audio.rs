#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_NPCTalkDialogueTextAudio: LazyLock<Vec<NPCTalkDialogueTextAudioRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/npctalkdialoguetextaudio.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| NPCTalkDialogueTextAudioRow {
                r#id: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(0..0 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#target_character: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(8..8 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    CharactersRef::new(value as usize)
                },
                r#text: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(24..24 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#ogg_files: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(32..32 + 16).unwrap();
                    let count = cell_bytes.get_u64_le() as usize;
                    let offset = cell_bytes.get_u64_le() as usize;
                    // array_mutator column.array == true && column.type == 'string'
                    let values = df.strings_from_offset(offset, count).unwrap();
                    values
                },
                r#unknown48: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(48..48 + 16).unwrap();
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
                r#unknown64: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(64..64 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    value
                },
                r#target_characters: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(80..80 + 16).unwrap();
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
                        .map(|value| CharactersRef::new(value as usize))
                        .collect()
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
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(104..104 + 16).unwrap();
                    let count = cell_bytes.get_u64_le() as usize;
                    let offset = cell_bytes.get_u64_le() as usize;
                    // array_mutator column.array == true && column.type == 'array'
                    let values = (count, offset);
                    values
                },
                r#unknown120: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(120..120 + 16).unwrap();
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
                r#unknown136: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(136..136 + 16).unwrap();
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
                r#unknown152: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(152..152 + 16).unwrap();
                    let count = cell_bytes.get_u64_le() as usize;
                    let offset = cell_bytes.get_u64_le() as usize;
                    // array_mutator column.array == true && column.type == 'string'
                    let values = df.strings_from_offset(offset, count).unwrap();
                    values
                },
                r#unknown168: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(168..168 + 16).unwrap();
                    let count = cell_bytes.get_u64_le() as usize;
                    let offset = cell_bytes.get_u64_le() as usize;
                    // array_mutator column.array == true && column.type == 'array'
                    let values = (count, offset);
                    values
                },
                r#unknown184: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(184..184 + 16).unwrap();
                    let count = cell_bytes.get_u64_le() as usize;
                    let offset = cell_bytes.get_u64_le() as usize;
                    // array_mutator column.array == true && column.type == 'array'
                    let values = (count, offset);
                    values
                },
                r#unknown200: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(200..200 + 16).unwrap();
                    let count = cell_bytes.get_u64_le() as usize;
                    let offset = cell_bytes.get_u64_le() as usize;
                    // array_mutator column.array == true && column.type == 'string'
                    let values = df.strings_from_offset(offset, count).unwrap();
                    values
                },
                r#unknown216: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(216..216 + 16).unwrap();
                    let count = cell_bytes.get_u64_le() as usize;
                    let offset = cell_bytes.get_u64_le() as usize;
                    // array_mutator column.array == true && column.type == 'string'
                    let values = df.strings_from_offset(offset, count).unwrap();
                    values
                },
                r#unknown232: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(232..232 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct NPCTalkDialogueTextAudioRow {
    pub r#id: String,
    pub r#target_character: CharactersRef,
    pub r#text: String,
    pub r#ogg_files: Vec<String>,
    pub r#unknown48: Vec<i64>,
    pub r#unknown64: i64,
    pub r#target_characters: Vec<CharactersRef>,
    pub r#unknown96: i32,
    pub r#unknown100: i32,
    pub r#unknown104: (usize, usize),
    pub r#unknown120: Vec<i64>,
    pub r#unknown136: Vec<i64>,
    pub r#unknown152: Vec<String>,
    pub r#unknown168: (usize, usize),
    pub r#unknown184: (usize, usize),
    pub r#unknown200: Vec<String>,
    pub r#unknown216: Vec<String>,
    pub r#unknown232: i64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct NPCTalkDialogueTextAudioRef(pub usize);

impl Deref for NPCTalkDialogueTextAudioRef {
    type Target = NPCTalkDialogueTextAudioRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_NPCTalkDialogueTextAudio[self.0]
    }
}

impl NPCTalkDialogueTextAudioRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static NPCTalkDialogueTextAudioRow {
        &TABLE_NPCTalkDialogueTextAudio[self.0]
    }
    pub fn get(&self) -> &'static NPCTalkDialogueTextAudioRow {
        &TABLE_NPCTalkDialogueTextAudio[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_NPCTalkDialogueTextAudio
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static NPCTalkDialogueTextAudioRow)> {
        TABLE_NPCTalkDialogueTextAudio
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
        for row in TABLE_NPCTalkDialogueTextAudio.iter() {
            black_box(row);
        }
    }
}
