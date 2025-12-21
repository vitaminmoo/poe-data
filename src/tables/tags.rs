#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_Tags: LazyLock<Vec<TagsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/tags.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| TagsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#hash32: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 4).unwrap();
                let value = cell_bytes.get_u32_le();
                value
            },
            r#display_string: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(12..12 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown20: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(20..20 + 16).unwrap();
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
                    .map(|value| ClientStringsRef::new(value as usize))
                    .collect()
            },
            r#ui_hints: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(36..36 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct TagsRow {
    pub r#id: String,
    pub r#hash32: u32,
    pub r#display_string: String,
    pub r#unknown20: Vec<ClientStringsRef>,
    pub r#ui_hints: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct TagsRef(pub usize);

impl Deref for TagsRef {
    type Target = TagsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_Tags[self.0]
    }
}

impl TagsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static TagsRow {
        &TABLE_Tags[self.0]
    }
    pub fn get(&self) -> &'static TagsRow {
        &TABLE_Tags[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Tags.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static TagsRow)> {
        TABLE_Tags.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_Tags.iter() {
            black_box(row);
        }
    }
}
