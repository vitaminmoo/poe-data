#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ItemFrameType: LazyLock<Vec<ItemFrameTypeRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/itemframetype.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ItemFrameTypeRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown8: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(8).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#double_line: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(9).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#header_single: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(10..10 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#header_double: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(18..18 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#hardmode_header_single: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(26..26 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#hardmode_header_double: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(34..34 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#color: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(42..42 + 16).unwrap();
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
            r#separator: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(58..58 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown66: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(66).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#rarity: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(67..67 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                RarityRef::new(value as usize)
            },
            r#display_string: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(83..83 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ClientStringsRef::new(value as usize)
            },
            r#color_markup: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(99..99 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ItemFrameTypeRow {
    pub r#id: String,
    pub r#unknown8: bool,
    pub r#double_line: bool,
    pub r#header_single: String,
    pub r#header_double: String,
    pub r#hardmode_header_single: String,
    pub r#hardmode_header_double: String,
    pub r#color: Vec<i32>,
    pub r#separator: String,
    pub r#unknown66: bool,
    pub r#rarity: RarityRef,
    pub r#display_string: ClientStringsRef,
    pub r#color_markup: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ItemFrameTypeRef(pub usize);

impl Deref for ItemFrameTypeRef {
    type Target = ItemFrameTypeRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ItemFrameType[self.0]
    }
}

impl ItemFrameTypeRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ItemFrameTypeRow {
        &TABLE_ItemFrameType[self.0]
    }
    pub fn get(&self) -> &'static ItemFrameTypeRow {
        &TABLE_ItemFrameType[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ItemFrameType.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ItemFrameTypeRow)> {
        TABLE_ItemFrameType
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
        for row in TABLE_ItemFrameType.iter() {
            black_box(row);
        }
    }
}
