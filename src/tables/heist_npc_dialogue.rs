#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_HeistNPCDialogue: LazyLock<Vec<HeistNPCDialogueRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/heistnpcdialogue.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| HeistNPCDialogueRow {
            r#dialogue_event_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                DialogueEventRef::new(value as usize)
            },
            r#heist_np_cs_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                HeistNPCsRef::new(value as usize)
            },
            r#audio_normal: {
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
                values.into_iter().map(|value| NPCTextAudioRef::new(value as usize)).collect()
            },
            r#audio_loud: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(48..48 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| NPCTextAudioRef::new(value as usize)).collect()
            },
            r#unknown64: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(64..64 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct HeistNPCDialogueRow {
    pub r#dialogue_event_key: DialogueEventRef,
    pub r#heist_np_cs_key: HeistNPCsRef,
    pub r#audio_normal: Vec<NPCTextAudioRef>,
    pub r#audio_loud: Vec<NPCTextAudioRef>,
    pub r#unknown64: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct HeistNPCDialogueRef(pub usize);

impl Deref for HeistNPCDialogueRef {
    type Target = HeistNPCDialogueRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_HeistNPCDialogue[self.0]
    }
}

impl HeistNPCDialogueRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static HeistNPCDialogueRow {
        &TABLE_HeistNPCDialogue[self.0]
    }
    pub fn get(&self) -> &'static HeistNPCDialogueRow {
        &TABLE_HeistNPCDialogue[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_HeistNPCDialogue.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static HeistNPCDialogueRow)> {
        TABLE_HeistNPCDialogue.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_HeistNPCDialogue.iter() {
            black_box(row);
        }
    }
}
