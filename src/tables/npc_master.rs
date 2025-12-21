#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_NPCMaster: LazyLock<Vec<NPCMasterRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/npcmaster.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| NPCMasterRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown8: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(8).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown9: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(9).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#signature_mods_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(10..10 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ModsRef::new(value as usize)
            },
            r#unknown26: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(26).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#spawn_weight_tags_keys: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(27..27 + 16).unwrap();
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
                    .map(|value| TagsRef::new(value as usize))
                    .collect()
            },
            r#spawn_weight_values: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(43..43 + 16).unwrap();
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
            r#unknown59: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(59..59 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown75: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(75..75 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#area_description: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(79..79 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown87: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(87..87 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown103: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(103..103 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown107: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(107..107 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#has_area_missions: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(123).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown124: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(124..124 + 16).unwrap();
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
            r#unknown140: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(140..140 + 16).unwrap();
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
            r#unknown156: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(156..156 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown172: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(172..172 + 16).unwrap();
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
            r#unknown188: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(188..188 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct NPCMasterRow {
    pub r#id: String,
    pub r#unknown8: bool,
    pub r#unknown9: bool,
    pub r#signature_mods_key: ModsRef,
    pub r#unknown26: bool,
    pub r#spawn_weight_tags_keys: Vec<TagsRef>,
    pub r#spawn_weight_values: Vec<i32>,
    pub r#unknown59: i64,
    pub r#unknown75: i32,
    pub r#area_description: String,
    pub r#unknown87: i64,
    pub r#unknown103: i32,
    pub r#unknown107: StatsRef,
    pub r#has_area_missions: bool,
    pub r#unknown124: Vec<i64>,
    pub r#unknown140: Vec<i64>,
    pub r#unknown156: i64,
    pub r#unknown172: Vec<i64>,
    pub r#unknown188: i64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct NPCMasterRef(pub usize);

impl Deref for NPCMasterRef {
    type Target = NPCMasterRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_NPCMaster[self.0]
    }
}

impl NPCMasterRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static NPCMasterRow {
        &TABLE_NPCMaster[self.0]
    }
    pub fn get(&self) -> &'static NPCMasterRow {
        &TABLE_NPCMaster[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_NPCMaster.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static NPCMasterRow)> {
        TABLE_NPCMaster
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
        for row in TABLE_NPCMaster.iter() {
            black_box(row);
        }
    }
}
