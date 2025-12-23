#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_UITalkText: LazyLock<Vec<UITalkTextRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/uitalktext.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| UITalkTextRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#ui_talk_categories_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                UITalkCategories::from_repr(value as usize)
            },
            r#ogg_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(12..12 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#text: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(20..20 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown28: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(28).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#npc_text_audio: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(29..29 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                NPCTextAudioRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct UITalkTextRow {
    pub r#id: String,
    pub r#ui_talk_categories_key: Option<UITalkCategories>,
    pub r#ogg_file: String,
    pub r#text: String,
    pub r#unknown28: bool,
    pub r#npc_text_audio: NPCTextAudioRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct UITalkTextRef(pub usize);

impl Deref for UITalkTextRef {
    type Target = UITalkTextRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_UITalkText[self.0]
    }
}

impl UITalkTextRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static UITalkTextRow {
        &TABLE_UITalkText[self.0]
    }
    pub fn get(&self) -> &'static UITalkTextRow {
        &TABLE_UITalkText[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_UITalkText.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static UITalkTextRow)> {
        TABLE_UITalkText.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_UITalkText.iter() {
            black_box(row);
        }
    }
}
