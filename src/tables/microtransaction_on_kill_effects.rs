#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MicrotransactionOnKillEffects: LazyLock<Vec<MicrotransactionOnKillEffectsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/microtransactiononkilleffects.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| MicrotransactionOnKillEffectsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown8: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown24: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown40: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown56: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(56..56 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown60: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(60..60 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown76: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(76..76 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown80: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(80..80 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MicrotransactionOnKillEffectsRow {
    pub r#id: String,
    pub r#unknown8: i64,
    pub r#unknown24: i64,
    pub r#unknown40: i64,
    pub r#unknown56: i32,
    pub r#unknown60: i64,
    pub r#unknown76: i32,
    pub r#unknown80: i64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MicrotransactionOnKillEffectsRef(pub usize);

impl Deref for MicrotransactionOnKillEffectsRef {
    type Target = MicrotransactionOnKillEffectsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MicrotransactionOnKillEffects[self.0]
    }
}

impl MicrotransactionOnKillEffectsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MicrotransactionOnKillEffectsRow {
        &TABLE_MicrotransactionOnKillEffects[self.0]
    }
    pub fn get(&self) -> &'static MicrotransactionOnKillEffectsRow {
        &TABLE_MicrotransactionOnKillEffects[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MicrotransactionOnKillEffects.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MicrotransactionOnKillEffectsRow)> {
        TABLE_MicrotransactionOnKillEffects.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_MicrotransactionOnKillEffects.iter() {
            black_box(row);
        }
    }
}
