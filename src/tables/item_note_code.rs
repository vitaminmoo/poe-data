#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ItemNoteCode: LazyLock<Vec<ItemNoteCodeRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/itemnotecode.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ItemNoteCodeRow {
            r#base_item: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#code: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#order1: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#show: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(28).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#order2: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(29..29 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ItemNoteCodeRow {
    pub r#base_item: BaseItemTypesRef,
    pub r#code: String,
    pub r#order1: i32,
    pub r#show: bool,
    pub r#order2: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ItemNoteCodeRef(pub usize);

impl Deref for ItemNoteCodeRef {
    type Target = ItemNoteCodeRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ItemNoteCode[self.0]
    }
}

impl ItemNoteCodeRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ItemNoteCodeRow {
        &TABLE_ItemNoteCode[self.0]
    }
    pub fn get(&self) -> &'static ItemNoteCodeRow {
        &TABLE_ItemNoteCode[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ItemNoteCode.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ItemNoteCodeRow)> {
        TABLE_ItemNoteCode
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
        for row in TABLE_ItemNoteCode.iter() {
            black_box(row);
        }
    }
}
