#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_PlayerTradeWhisperFormats: LazyLock<Vec<PlayerTradeWhisperFormatsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/playertradewhisperformats.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| PlayerTradeWhisperFormatsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#whisper: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#in_stash: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(16).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#is_priced: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(17).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct PlayerTradeWhisperFormatsRow {
    pub r#id: String,
    pub r#whisper: String,
    pub r#in_stash: bool,
    pub r#is_priced: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct PlayerTradeWhisperFormatsRef(pub usize);

impl Deref for PlayerTradeWhisperFormatsRef {
    type Target = PlayerTradeWhisperFormatsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_PlayerTradeWhisperFormats[self.0]
    }
}

impl PlayerTradeWhisperFormatsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static PlayerTradeWhisperFormatsRow {
        &TABLE_PlayerTradeWhisperFormats[self.0]
    }
    pub fn get(&self) -> &'static PlayerTradeWhisperFormatsRow {
        &TABLE_PlayerTradeWhisperFormats[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_PlayerTradeWhisperFormats.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static PlayerTradeWhisperFormatsRow)> {
        TABLE_PlayerTradeWhisperFormats.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_PlayerTradeWhisperFormats.iter() {
            black_box(row);
        }
    }
}
