#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_QuestFlags: LazyLock<Vec<QuestFlagsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/questflags.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| QuestFlagsRow {
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
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct QuestFlagsRow {
    pub r#id: String,
    pub r#hash32: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct QuestFlagsRef(pub usize);

impl Deref for QuestFlagsRef {
    type Target = QuestFlagsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_QuestFlags[self.0]
    }
}

impl QuestFlagsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static QuestFlagsRow {
        &TABLE_QuestFlags[self.0]
    }
    pub fn get(&self) -> &'static QuestFlagsRow {
        &TABLE_QuestFlags[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_QuestFlags.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static QuestFlagsRow)> {
        TABLE_QuestFlags.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_QuestFlags.iter() {
            black_box(row);
        }
    }
}
