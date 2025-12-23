#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_BlightEncounterWaves: LazyLock<Vec<BlightEncounterWavesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/blightencounterwaves.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| BlightEncounterWavesRow {
            r#monster_spawner_id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#encounter_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BlightEncounterTypesRef::new(value as usize)
            },
            r#unknown24: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown28: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(28..28 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown32: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#wave: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(36..36 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct BlightEncounterWavesRow {
    pub r#monster_spawner_id: String,
    pub r#encounter_type: BlightEncounterTypesRef,
    pub r#unknown24: i32,
    pub r#unknown28: i32,
    pub r#unknown32: i32,
    pub r#wave: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BlightEncounterWavesRef(pub usize);

impl Deref for BlightEncounterWavesRef {
    type Target = BlightEncounterWavesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_BlightEncounterWaves[self.0]
    }
}

impl BlightEncounterWavesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BlightEncounterWavesRow {
        &TABLE_BlightEncounterWaves[self.0]
    }
    pub fn get(&self) -> &'static BlightEncounterWavesRow {
        &TABLE_BlightEncounterWaves[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BlightEncounterWaves.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BlightEncounterWavesRow)> {
        TABLE_BlightEncounterWaves.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_BlightEncounterWaves.iter() {
            black_box(row);
        }
    }
}
