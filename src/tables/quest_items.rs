#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_QuestItems: LazyLock<Vec<QuestItemsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/questitems.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| QuestItemsRow {
            r#base_item_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#triggered_quest_flag: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                QuestFlagsRef::new(value as usize)
            },
            r#unknown32: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                QuestFlagsRef::new(value as usize)
            },
            r#unknown48: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(48..48 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown52: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(52..52 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown68: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(68).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown69: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(69).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#help_text: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(70..70 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ClientStringsRef::new(value as usize)
            },
            r#description: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(86..86 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ClientStringsRef::new(value as usize)
            },
            r#script: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(102..102 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown110: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(110..110 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown126: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(126..126 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown142: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(142..142 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown146: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(146..146 + 16).unwrap();
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
            r#unknown162: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(162..162 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                QuestFlagsRef::new(value as usize)
            },
            r#unknown178: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(178..178 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct QuestItemsRow {
    pub r#base_item_type: BaseItemTypesRef,
    pub r#triggered_quest_flag: QuestFlagsRef,
    pub r#unknown32: QuestFlagsRef,
    pub r#unknown48: i32,
    pub r#unknown52: (usize, usize),
    pub r#unknown68: bool,
    pub r#unknown69: bool,
    pub r#help_text: ClientStringsRef,
    pub r#description: ClientStringsRef,
    pub r#script: String,
    pub r#unknown110: i64,
    pub r#unknown126: i64,
    pub r#unknown142: i32,
    pub r#unknown146: Vec<i64>,
    pub r#unknown162: QuestFlagsRef,
    pub r#unknown178: i64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct QuestItemsRef(pub usize);

impl Deref for QuestItemsRef {
    type Target = QuestItemsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_QuestItems[self.0]
    }
}

impl QuestItemsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static QuestItemsRow {
        &TABLE_QuestItems[self.0]
    }
    pub fn get(&self) -> &'static QuestItemsRow {
        &TABLE_QuestItems[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_QuestItems.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static QuestItemsRow)> {
        TABLE_QuestItems.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_QuestItems.iter() {
            black_box(row);
        }
    }
}
