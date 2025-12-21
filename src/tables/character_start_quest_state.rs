#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_CharacterStartQuestState: LazyLock<Vec<CharacterStartQuestStateRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/characterstartqueststate.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| CharacterStartQuestStateRow {
                r#id: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(0..0 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#quest_keys: {
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
                        .into_iter()
                        .map(|value| QuestRef::new(value as usize))
                        .collect()
                },
                r#quest_states: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(24..24 + 16).unwrap();
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
                r#unknown40: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(40..40 + 16).unwrap();
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
                r#map_pins_keys: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(56..56 + 16).unwrap();
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
                        .into_iter()
                        .map(|value| MapPinsRef::new(value as usize))
                        .collect()
                },
                r#unknown72: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(72..72 + 16).unwrap();
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
                r#unknown88: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(88..88 + 16).unwrap();
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
pub struct CharacterStartQuestStateRow {
    pub r#id: String,
    pub r#quest_keys: Vec<QuestRef>,
    pub r#quest_states: Vec<i32>,
    pub r#unknown40: Vec<i64>,
    pub r#map_pins_keys: Vec<MapPinsRef>,
    pub r#unknown72: Vec<i32>,
    pub r#unknown88: Vec<i64>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct CharacterStartQuestStateRef(pub usize);

impl Deref for CharacterStartQuestStateRef {
    type Target = CharacterStartQuestStateRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_CharacterStartQuestState[self.0]
    }
}

impl CharacterStartQuestStateRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static CharacterStartQuestStateRow {
        &TABLE_CharacterStartQuestState[self.0]
    }
    pub fn get(&self) -> &'static CharacterStartQuestStateRow {
        &TABLE_CharacterStartQuestState[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_CharacterStartQuestState
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static CharacterStartQuestStateRow)> {
        TABLE_CharacterStartQuestState
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
        for row in TABLE_CharacterStartQuestState.iter() {
            black_box(row);
        }
    }
}
