#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_HeistRevealingNPCs: LazyLock<Vec<HeistRevealingNPCsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/heistrevealingnpcs.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| HeistRevealingNPCsRow {
            r#np_cs_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                NPCsRef::new(value as usize)
            },
            r#portrait_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#npc_audio_key: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
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
            r#unknown40: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct HeistRevealingNPCsRow {
    pub r#np_cs_key: NPCsRef,
    pub r#portrait_file: String,
    pub r#npc_audio_key: Vec<NPCAudioRef>,
    pub r#unknown40: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct HeistRevealingNPCsRef(pub usize);

impl Deref for HeistRevealingNPCsRef {
    type Target = HeistRevealingNPCsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_HeistRevealingNPCs[self.0]
    }
}

impl HeistRevealingNPCsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static HeistRevealingNPCsRow {
        &TABLE_HeistRevealingNPCs[self.0]
    }
    pub fn get(&self) -> &'static HeistRevealingNPCsRow {
        &TABLE_HeistRevealingNPCs[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_HeistRevealingNPCs
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static HeistRevealingNPCsRow)> {
        TABLE_HeistRevealingNPCs
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
        for row in TABLE_HeistRevealingNPCs.iter() {
            black_box(row);
        }
    }
}
