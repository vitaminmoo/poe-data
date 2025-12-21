#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_QuestRewardType: LazyLock<Vec<QuestRewardTypeRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/questrewardtype.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| QuestRewardTypeRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#icon_dds_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#description: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(24..24 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#reward: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#quest_flag: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(48..48 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                QuestFlagsRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct QuestRewardTypeRow {
    pub r#id: String,
    pub r#icon_dds_file: String,
    pub r#name: String,
    pub r#description: String,
    pub r#reward: BaseItemTypesRef,
    pub r#quest_flag: QuestFlagsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct QuestRewardTypeRef(pub usize);

impl Deref for QuestRewardTypeRef {
    type Target = QuestRewardTypeRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_QuestRewardType[self.0]
    }
}

impl QuestRewardTypeRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static QuestRewardTypeRow {
        &TABLE_QuestRewardType[self.0]
    }
    pub fn get(&self) -> &'static QuestRewardTypeRow {
        &TABLE_QuestRewardType[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_QuestRewardType
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static QuestRewardTypeRow)> {
        TABLE_QuestRewardType
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
        for row in TABLE_QuestRewardType.iter() {
            black_box(row);
        }
    }
}
