#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_UltimatumEncounterTypes: LazyLock<Vec<UltimatumEncounterTypesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/ultimatumencountertypes.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| UltimatumEncounterTypesRow {
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
            r#unknown16: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(16).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown17: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(17).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#quest_flag: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(18..18 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                QuestFlagsRef::new(value as usize)
            },
            r#npc_text: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(34..34 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                NPCTextAudioRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct UltimatumEncounterTypesRow {
    pub r#id: String,
    pub r#name: String,
    pub r#unknown16: bool,
    pub r#unknown17: bool,
    pub r#quest_flag: QuestFlagsRef,
    pub r#npc_text: NPCTextAudioRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct UltimatumEncounterTypesRef(pub usize);

impl Deref for UltimatumEncounterTypesRef {
    type Target = UltimatumEncounterTypesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_UltimatumEncounterTypes[self.0]
    }
}

impl UltimatumEncounterTypesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static UltimatumEncounterTypesRow {
        &TABLE_UltimatumEncounterTypes[self.0]
    }
    pub fn get(&self) -> &'static UltimatumEncounterTypesRow {
        &TABLE_UltimatumEncounterTypes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_UltimatumEncounterTypes.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static UltimatumEncounterTypesRow)> {
        TABLE_UltimatumEncounterTypes.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_UltimatumEncounterTypes.iter() {
            black_box(row);
        }
    }
}
