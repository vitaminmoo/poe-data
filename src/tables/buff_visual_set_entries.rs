#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_BuffVisualSetEntries: LazyLock<Vec<BuffVisualSetEntriesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/buffvisualsetentries.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| BuffVisualSetEntriesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown8: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#buff_visual: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(12..12 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BuffVisualsRef::new(value as usize)
            },
            r#unknown28: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(28..28 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct BuffVisualSetEntriesRow {
    pub r#id: String,
    pub r#unknown8: i32,
    pub r#buff_visual: BuffVisualsRef,
    pub r#unknown28: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BuffVisualSetEntriesRef(pub usize);

impl Deref for BuffVisualSetEntriesRef {
    type Target = BuffVisualSetEntriesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_BuffVisualSetEntries[self.0]
    }
}

impl BuffVisualSetEntriesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BuffVisualSetEntriesRow {
        &TABLE_BuffVisualSetEntries[self.0]
    }
    pub fn get(&self) -> &'static BuffVisualSetEntriesRow {
        &TABLE_BuffVisualSetEntries[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BuffVisualSetEntries.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BuffVisualSetEntriesRow)> {
        TABLE_BuffVisualSetEntries.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_BuffVisualSetEntries.iter() {
            black_box(row);
        }
    }
}
