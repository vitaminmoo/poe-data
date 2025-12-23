#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MiniQuestStates: LazyLock<Vec<MiniQuestStatesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/miniqueststates.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| MiniQuestStatesRow {
            r#unknown0: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#quest_flags1: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(4..4 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| QuestFlagsRef::new(value as usize)).collect()
            },
            r#quest_flags2: {
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
                values.into_iter().map(|value| QuestFlagsRef::new(value as usize)).collect()
            },
            r#unknown36: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(36..36 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown44: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(44..44 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#tsi_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(48..48 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MiniQuestStatesRow {
    pub r#unknown0: i32,
    pub r#quest_flags1: Vec<QuestFlagsRef>,
    pub r#quest_flags2: Vec<QuestFlagsRef>,
    pub r#unknown36: String,
    pub r#unknown44: i32,
    pub r#tsi_file: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MiniQuestStatesRef(pub usize);

impl Deref for MiniQuestStatesRef {
    type Target = MiniQuestStatesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MiniQuestStates[self.0]
    }
}

impl MiniQuestStatesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MiniQuestStatesRow {
        &TABLE_MiniQuestStates[self.0]
    }
    pub fn get(&self) -> &'static MiniQuestStatesRow {
        &TABLE_MiniQuestStates[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MiniQuestStates.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MiniQuestStatesRow)> {
        TABLE_MiniQuestStates.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_MiniQuestStates.iter() {
            black_box(row);
        }
    }
}
