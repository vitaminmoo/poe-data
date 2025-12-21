#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_CharacterCreationDialogue: LazyLock<Vec<CharacterCreationDialogueRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/charactercreationdialogue.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| CharacterCreationDialogueRow {
                r#character: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(0..0 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    CharactersRef::new(value as usize)
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
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(48..48 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct CharacterCreationDialogueRow {
    pub r#character: CharactersRef,
    pub r#unknown16: i64,
    pub r#unknown32: i64,
    pub r#unknown48: i64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct CharacterCreationDialogueRef(pub usize);

impl Deref for CharacterCreationDialogueRef {
    type Target = CharacterCreationDialogueRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_CharacterCreationDialogue[self.0]
    }
}

impl CharacterCreationDialogueRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static CharacterCreationDialogueRow {
        &TABLE_CharacterCreationDialogue[self.0]
    }
    pub fn get(&self) -> &'static CharacterCreationDialogueRow {
        &TABLE_CharacterCreationDialogue[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_CharacterCreationDialogue
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static CharacterCreationDialogueRow)> {
        TABLE_CharacterCreationDialogue
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
        for row in TABLE_CharacterCreationDialogue.iter() {
            black_box(row);
        }
    }
}
