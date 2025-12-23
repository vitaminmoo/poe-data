#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_HideoutNPCs: LazyLock<Vec<HideoutNPCsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/hideoutnpcs.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| HideoutNPCsRow {
            r#hideout_npc: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                NPCsRef::new(value as usize)
            },
            r#hideout_doodad: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                HideoutDoodadsRef::new(value as usize)
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
            r#unknown64: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(64..64 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown68: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(68..68 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown84: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(84..84 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown88: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(88..88 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct HideoutNPCsRow {
    pub r#hideout_npc: NPCsRef,
    pub r#hideout_doodad: HideoutDoodadsRef,
    pub r#unknown32: i64,
    pub r#unknown48: i64,
    pub r#unknown64: i32,
    pub r#unknown68: i64,
    pub r#unknown84: i32,
    pub r#unknown88: i64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct HideoutNPCsRef(pub usize);

impl Deref for HideoutNPCsRef {
    type Target = HideoutNPCsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_HideoutNPCs[self.0]
    }
}

impl HideoutNPCsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static HideoutNPCsRow {
        &TABLE_HideoutNPCs[self.0]
    }
    pub fn get(&self) -> &'static HideoutNPCsRow {
        &TABLE_HideoutNPCs[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_HideoutNPCs.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static HideoutNPCsRow)> {
        TABLE_HideoutNPCs.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_HideoutNPCs.iter() {
            black_box(row);
        }
    }
}
