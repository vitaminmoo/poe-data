#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ItemVisualReplacement: LazyLock<Vec<ItemVisualReplacementRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/itemvisualreplacement.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ItemVisualReplacementRow {
            r#unknown0: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown32: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown48: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(48).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown49: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(49).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown50: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(50).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown51: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(51).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ItemVisualReplacementRow {
    pub r#unknown0: i64,
    pub r#unknown16: i64,
    pub r#unknown32: i64,
    pub r#unknown48: bool,
    pub r#unknown49: bool,
    pub r#unknown50: bool,
    pub r#unknown51: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ItemVisualReplacementRef(pub usize);

impl Deref for ItemVisualReplacementRef {
    type Target = ItemVisualReplacementRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ItemVisualReplacement[self.0]
    }
}

impl ItemVisualReplacementRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ItemVisualReplacementRow {
        &TABLE_ItemVisualReplacement[self.0]
    }
    pub fn get(&self) -> &'static ItemVisualReplacementRow {
        &TABLE_ItemVisualReplacement[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ItemVisualReplacement.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ItemVisualReplacementRow)> {
        TABLE_ItemVisualReplacement.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_ItemVisualReplacement.iter() {
            black_box(row);
        }
    }
}
