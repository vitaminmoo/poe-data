#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_NPCTalkDialogueSteps: LazyLock<Vec<NPCTalkDialogueStepsRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/npctalkdialoguesteps.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| NPCTalkDialogueStepsRow {
                r#npc_talk_dialogue: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(0..0 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    NPCTalkDialogueRef::new(value as usize)
                },
                r#step: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(16..16 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#npc_text_audio: {
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
                    values
                        .into_iter()
                        .map(|value| NPCTextAudioRef::new(value as usize))
                        .collect()
                },
                r#npc_talk_dialogue_text_audio: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(36..36 + 16).unwrap();
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
                        .map(|value| NPCTalkDialogueTextAudioRef::new(value as usize))
                        .collect()
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct NPCTalkDialogueStepsRow {
    pub r#npc_talk_dialogue: NPCTalkDialogueRef,
    pub r#step: i32,
    pub r#npc_text_audio: Vec<NPCTextAudioRef>,
    pub r#npc_talk_dialogue_text_audio: Vec<NPCTalkDialogueTextAudioRef>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct NPCTalkDialogueStepsRef(pub usize);

impl Deref for NPCTalkDialogueStepsRef {
    type Target = NPCTalkDialogueStepsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_NPCTalkDialogueSteps[self.0]
    }
}

impl NPCTalkDialogueStepsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static NPCTalkDialogueStepsRow {
        &TABLE_NPCTalkDialogueSteps[self.0]
    }
    pub fn get(&self) -> &'static NPCTalkDialogueStepsRow {
        &TABLE_NPCTalkDialogueSteps[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_NPCTalkDialogueSteps
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static NPCTalkDialogueStepsRow)> {
        TABLE_NPCTalkDialogueSteps
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
        for row in TABLE_NPCTalkDialogueSteps.iter() {
            black_box(row);
        }
    }
}
