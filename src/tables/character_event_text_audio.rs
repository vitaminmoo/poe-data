#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_CharacterEventTextAudio: LazyLock<Vec<CharacterEventTextAudioRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/charactereventtextaudio.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| CharacterEventTextAudioRow {
            r#event: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                CharacterAudioEventsRef::new(value as usize)
            },
            r#character: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                CharactersRef::new(value as usize)
            },
            r#text_audio: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| CharacterTextAudioRef::new(value as usize)).collect()
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct CharacterEventTextAudioRow {
    pub r#event: CharacterAudioEventsRef,
    pub r#character: CharactersRef,
    pub r#text_audio: Vec<CharacterTextAudioRef>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct CharacterEventTextAudioRef(pub usize);

impl Deref for CharacterEventTextAudioRef {
    type Target = CharacterEventTextAudioRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_CharacterEventTextAudio[self.0]
    }
}

impl CharacterEventTextAudioRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static CharacterEventTextAudioRow {
        &TABLE_CharacterEventTextAudio[self.0]
    }
    pub fn get(&self) -> &'static CharacterEventTextAudioRow {
        &TABLE_CharacterEventTextAudio[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_CharacterEventTextAudio.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static CharacterEventTextAudioRow)> {
        TABLE_CharacterEventTextAudio.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_CharacterEventTextAudio.iter() {
            black_box(row);
        }
    }
}
