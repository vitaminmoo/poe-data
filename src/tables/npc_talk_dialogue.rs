#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_NPCTalkDialogue: LazyLock<Vec<NPCTalkDialogueRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/npctalkdialogue.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| NPCTalkDialogueRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown8: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
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
            r#unknown24: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown40: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct NPCTalkDialogueRow {
    pub r#id: String,
    pub r#unknown8: Vec<i64>,
    pub r#unknown24: Vec<String>,
    pub r#unknown40: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct NPCTalkDialogueRef(pub usize);

impl Deref for NPCTalkDialogueRef {
    type Target = NPCTalkDialogueRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_NPCTalkDialogue[self.0]
    }
}

impl NPCTalkDialogueRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static NPCTalkDialogueRow {
        &TABLE_NPCTalkDialogue[self.0]
    }
    pub fn get(&self) -> &'static NPCTalkDialogueRow {
        &TABLE_NPCTalkDialogue[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_NPCTalkDialogue
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static NPCTalkDialogueRow)> {
        TABLE_NPCTalkDialogue
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
        for row in TABLE_NPCTalkDialogue.iter() {
            black_box(row);
        }
    }
}
