#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_UltimatumTrialMasterAudio: LazyLock<Vec<UltimatumTrialMasterAudioRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/ultimatumtrialmasteraudio.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| UltimatumTrialMasterAudioRow {
                r#id: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(0..0 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#variant: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(8..8 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown12: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(12..12 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown16: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(16..16 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#text_audio: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(20..20 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    NPCTextAudioRef::new(value as usize)
                },
                r#rounds_min: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(36..36 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#rounds_max: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(40..40 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct UltimatumTrialMasterAudioRow {
    pub r#id: String,
    pub r#variant: i32,
    pub r#unknown12: i32,
    pub r#unknown16: i32,
    pub r#text_audio: NPCTextAudioRef,
    pub r#rounds_min: i32,
    pub r#rounds_max: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct UltimatumTrialMasterAudioRef(pub usize);

impl Deref for UltimatumTrialMasterAudioRef {
    type Target = UltimatumTrialMasterAudioRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_UltimatumTrialMasterAudio[self.0]
    }
}

impl UltimatumTrialMasterAudioRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static UltimatumTrialMasterAudioRow {
        &TABLE_UltimatumTrialMasterAudio[self.0]
    }
    pub fn get(&self) -> &'static UltimatumTrialMasterAudioRow {
        &TABLE_UltimatumTrialMasterAudio[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_UltimatumTrialMasterAudio
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static UltimatumTrialMasterAudioRow)> {
        TABLE_UltimatumTrialMasterAudio
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
        for row in TABLE_UltimatumTrialMasterAudio.iter() {
            black_box(row);
        }
    }
}
