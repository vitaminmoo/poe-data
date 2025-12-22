#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_NPCs: LazyLock<Vec<NPCsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/npcs.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| NPCsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#metadata: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown24: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#npc_master_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#short_name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(56..56 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown64: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(64..64 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#npc_audios1: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(68..68 + 16).unwrap();
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
                    .map(|value| NPCAudioRef::new(value as usize))
                    .collect()
            },
            r#npc_audios2: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(84..84 + 16).unwrap();
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
                    .map(|value| NPCAudioRef::new(value as usize))
                    .collect()
            },
            r#hash16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(100..100 + 2).unwrap();
                let value = cell_bytes.get_i16_le();
                value
            },
            r#unknown102: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(102..102 + 8).unwrap();
                let value = cell_bytes.get_i64_le();
                NPCsRef::new(value as usize)
            },
            r#portrait: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(110..110 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                NPCPortraitsRef::new(value as usize)
            },
            r#dialogue_style: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(126..126 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                NPCDialogueStylesRef::new(value as usize)
            },
            r#unknown142: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(142..142 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#gender: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(158..158 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown166: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(166).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown167: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(167..167 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown175: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(175..175 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct NPCsRow {
    pub r#id: String,
    pub r#name: String,
    pub r#metadata: String,
    pub r#unknown24: i64,
    pub r#npc_master_key: i64,
    pub r#short_name: String,
    pub r#unknown64: i32,
    pub r#npc_audios1: Vec<NPCAudioRef>,
    pub r#npc_audios2: Vec<NPCAudioRef>,
    pub r#hash16: i16,
    pub r#unknown102: NPCsRef,
    pub r#portrait: NPCPortraitsRef,
    pub r#dialogue_style: NPCDialogueStylesRef,
    pub r#unknown142: i64,
    pub r#gender: String,
    pub r#unknown166: bool,
    pub r#unknown167: String,
    pub r#unknown175: i64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct NPCsRef(pub usize);

impl Deref for NPCsRef {
    type Target = NPCsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_NPCs[self.0]
    }
}

impl NPCsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static NPCsRow {
        &TABLE_NPCs[self.0]
    }
    pub fn get(&self) -> &'static NPCsRow {
        &TABLE_NPCs[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_NPCs.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static NPCsRow)> {
        TABLE_NPCs.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_NPCs.iter() {
            black_box(row);
        }
    }
}
