#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_Quest: LazyLock<Vec<QuestRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/quest.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| QuestRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#act: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(12..12 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#icon_dds_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(20..20 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#quest_id: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(28..28 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown32: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(32).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(33..33 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                QuestTypeRef::new(value as usize)
            },
            r#unknown49: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(49..49 + 16).unwrap();
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
            r#unknown65: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(65..65 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#tracker_group: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(69..69 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                QuestTrackerGroupRef::new(value as usize)
            },
            r#unknown85: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(85).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown86: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(86).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown87: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(87..87 + 16).unwrap();
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
        })
        .collect()
});

#[derive(Debug)]
pub struct QuestRow {
    pub r#id: String,
    pub r#act: i32,
    pub r#name: String,
    pub r#icon_dds_file: String,
    pub r#quest_id: i32,
    pub r#unknown32: bool,
    pub r#type: QuestTypeRef,
    pub r#unknown49: Vec<i32>,
    pub r#unknown65: i32,
    pub r#tracker_group: QuestTrackerGroupRef,
    pub r#unknown85: bool,
    pub r#unknown86: bool,
    pub r#unknown87: Vec<i64>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct QuestRef(pub usize);

impl Deref for QuestRef {
    type Target = QuestRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_Quest[self.0]
    }
}

impl QuestRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static QuestRow {
        &TABLE_Quest[self.0]
    }
    pub fn get(&self) -> &'static QuestRow {
        &TABLE_Quest[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Quest.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static QuestRow)> {
        TABLE_Quest.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_Quest.iter() {
            black_box(row);
        }
    }
}
