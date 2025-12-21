#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ChatIcons: LazyLock<Vec<ChatIconsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/chaticons.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ChatIconsRow {
            r#icon: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#image: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ChatIconsRow {
    pub r#icon: String,
    pub r#image: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ChatIconsRef(pub usize);

impl Deref for ChatIconsRef {
    type Target = ChatIconsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ChatIcons[self.0]
    }
}

impl ChatIconsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ChatIconsRow {
        &TABLE_ChatIcons[self.0]
    }
    pub fn get(&self) -> &'static ChatIconsRow {
        &TABLE_ChatIcons[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ChatIcons.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ChatIconsRow)> {
        TABLE_ChatIcons
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
        for row in TABLE_ChatIcons.iter() {
            black_box(row);
        }
    }
}
