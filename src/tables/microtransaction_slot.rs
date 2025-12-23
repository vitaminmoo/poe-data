#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MicrotransactionSlot: LazyLock<Vec<MicrotransactionSlotRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/microtransactionslot.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| MicrotransactionSlotRow {
            r#id: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                MicrotransactionSlotId::from_repr(value as usize)
            },
            r#unknown4: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(4..4 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(20..20 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown28: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(28..28 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown44: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(44..44 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
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
                let mut cell_bytes = row.get(49..49 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown53: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(53..53 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown57: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(57).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MicrotransactionSlotRow {
    pub r#id: Option<MicrotransactionSlotId>,
    pub r#unknown4: i64,
    pub r#name: String,
    pub r#unknown28: i64,
    pub r#unknown44: i32,
    pub r#unknown48: bool,
    pub r#unknown49: i32,
    pub r#unknown53: i32,
    pub r#unknown57: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MicrotransactionSlotRef(pub usize);

impl Deref for MicrotransactionSlotRef {
    type Target = MicrotransactionSlotRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MicrotransactionSlot[self.0]
    }
}

impl MicrotransactionSlotRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MicrotransactionSlotRow {
        &TABLE_MicrotransactionSlot[self.0]
    }
    pub fn get(&self) -> &'static MicrotransactionSlotRow {
        &TABLE_MicrotransactionSlot[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MicrotransactionSlot.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MicrotransactionSlotRow)> {
        TABLE_MicrotransactionSlot.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_MicrotransactionSlot.iter() {
            black_box(row);
        }
    }
}
