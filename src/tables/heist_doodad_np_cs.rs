#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_HeistDoodadNPCs: LazyLock<Vec<HeistDoodadNPCsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/heistdoodadnpcs.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| HeistDoodadNPCsRow {
            r#np_cs_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                NPCsRef::new(value as usize)
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
                let mut cell_bytes = row.get(48..48 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown52: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(52..52 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown56: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(56..56 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#ao_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(60..60 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#stance: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(68..68 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#betrayal_targets_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(76..76 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BetrayalTargetsRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct HeistDoodadNPCsRow {
    pub r#np_cs_key: NPCsRef,
    pub r#unknown16: i64,
    pub r#unknown32: i64,
    pub r#unknown48: i32,
    pub r#unknown52: i32,
    pub r#unknown56: i32,
    pub r#ao_file: String,
    pub r#stance: String,
    pub r#betrayal_targets_key: BetrayalTargetsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct HeistDoodadNPCsRef(pub usize);

impl Deref for HeistDoodadNPCsRef {
    type Target = HeistDoodadNPCsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_HeistDoodadNPCs[self.0]
    }
}

impl HeistDoodadNPCsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static HeistDoodadNPCsRow {
        &TABLE_HeistDoodadNPCs[self.0]
    }
    pub fn get(&self) -> &'static HeistDoodadNPCsRow {
        &TABLE_HeistDoodadNPCs[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_HeistDoodadNPCs.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static HeistDoodadNPCsRow)> {
        TABLE_HeistDoodadNPCs.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_HeistDoodadNPCs.iter() {
            black_box(row);
        }
    }
}
