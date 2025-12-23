#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MicrotransactionOnKillBeams: LazyLock<Vec<MicrotransactionOnKillBeamsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/microtransactiononkillbeams.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| MicrotransactionOnKillBeamsRow {
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
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(24..24 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown32: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(32..32 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown40: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(40..40 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown48: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(48).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown49: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(49..49 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MicrotransactionOnKillBeamsRow {
    pub r#id: String,
    pub r#unknown8: i64,
    pub r#unknown24: String,
    pub r#unknown32: String,
    pub r#unknown40: String,
    pub r#unknown48: bool,
    pub r#unknown49: i64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MicrotransactionOnKillBeamsRef(pub usize);

impl Deref for MicrotransactionOnKillBeamsRef {
    type Target = MicrotransactionOnKillBeamsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MicrotransactionOnKillBeams[self.0]
    }
}

impl MicrotransactionOnKillBeamsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MicrotransactionOnKillBeamsRow {
        &TABLE_MicrotransactionOnKillBeams[self.0]
    }
    pub fn get(&self) -> &'static MicrotransactionOnKillBeamsRow {
        &TABLE_MicrotransactionOnKillBeams[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MicrotransactionOnKillBeams.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MicrotransactionOnKillBeamsRow)> {
        TABLE_MicrotransactionOnKillBeams.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_MicrotransactionOnKillBeams.iter() {
            black_box(row);
        }
    }
}
