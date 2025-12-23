#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_QuestItemNPCAudio: LazyLock<Vec<QuestItemNPCAudioRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/questitemnpcaudio.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| QuestItemNPCAudioRow {
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
                // array_mutator column.array == true
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
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
            },
            r#unknown48: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(48..48 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct QuestItemNPCAudioRow {
    pub r#unknown0: i64,
    pub r#unknown16: i64,
    pub r#unknown32: Vec<i64>,
    pub r#unknown48: i64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct QuestItemNPCAudioRef(pub usize);

impl Deref for QuestItemNPCAudioRef {
    type Target = QuestItemNPCAudioRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_QuestItemNPCAudio[self.0]
    }
}

impl QuestItemNPCAudioRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static QuestItemNPCAudioRow {
        &TABLE_QuestItemNPCAudio[self.0]
    }
    pub fn get(&self) -> &'static QuestItemNPCAudioRow {
        &TABLE_QuestItemNPCAudio[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_QuestItemNPCAudio.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static QuestItemNPCAudioRow)> {
        TABLE_QuestItemNPCAudio.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_QuestItemNPCAudio.iter() {
            black_box(row);
        }
    }
}
