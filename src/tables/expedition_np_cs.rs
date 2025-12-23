#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ExpeditionNPCs: LazyLock<Vec<ExpeditionNPCsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/expeditionnpcs.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| ExpeditionNPCsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#np_cs: {
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
                values.into_iter().map(|value| NPCsRef::new(value as usize)).collect()
            },
            r#reroll_item: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#unknown40: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown44: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(44..44 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown48: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(48..48 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                QuestFlagsRef::new(value as usize)
            },
            r#faction: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(64..64 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ExpeditionFactionsRef::new(value as usize)
            },
            r#reroll: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(80..80 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                NPCTextAudioRef::new(value as usize)
            },
            r#all_bombs_placed: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(96..96 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                NPCTextAudioRef::new(value as usize)
            },
            r#bomb_placed_remnant: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(112..112 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                NPCTextAudioRef::new(value as usize)
            },
            r#bomb_placed_treasure: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(128..128 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                NPCTextAudioRef::new(value as usize)
            },
            r#bomb_placed_monsters: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(144..144 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                NPCTextAudioRef::new(value as usize)
            },
            r#bomb_placed_generic: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(160..160 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                NPCTextAudioRef::new(value as usize)
            },
            r#encounter_complete: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(176..176 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                NPCTextAudioRef::new(value as usize)
            },
            r#unknown192: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(192..192 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown196: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(196..196 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ExpeditionNPCsRow {
    pub r#id: String,
    pub r#np_cs: Vec<NPCsRef>,
    pub r#reroll_item: BaseItemTypesRef,
    pub r#unknown40: i32,
    pub r#unknown44: i32,
    pub r#unknown48: QuestFlagsRef,
    pub r#faction: ExpeditionFactionsRef,
    pub r#reroll: NPCTextAudioRef,
    pub r#all_bombs_placed: NPCTextAudioRef,
    pub r#bomb_placed_remnant: NPCTextAudioRef,
    pub r#bomb_placed_treasure: NPCTextAudioRef,
    pub r#bomb_placed_monsters: NPCTextAudioRef,
    pub r#bomb_placed_generic: NPCTextAudioRef,
    pub r#encounter_complete: NPCTextAudioRef,
    pub r#unknown192: i32,
    pub r#unknown196: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ExpeditionNPCsRef(pub usize);

impl Deref for ExpeditionNPCsRef {
    type Target = ExpeditionNPCsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ExpeditionNPCs[self.0]
    }
}

impl ExpeditionNPCsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ExpeditionNPCsRow {
        &TABLE_ExpeditionNPCs[self.0]
    }
    pub fn get(&self) -> &'static ExpeditionNPCsRow {
        &TABLE_ExpeditionNPCs[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ExpeditionNPCs.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ExpeditionNPCsRow)> {
        TABLE_ExpeditionNPCs.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_ExpeditionNPCs.iter() {
            black_box(row);
        }
    }
}
