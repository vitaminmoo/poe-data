#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_CostTypes: LazyLock<Vec<CostTypesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/costtypes.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| CostTypesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#stat: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#format_text: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(24..24 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#divisor: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#per_minute: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(36).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct CostTypesRow {
    pub r#id: String,
    pub r#stat: StatsRef,
    pub r#format_text: String,
    pub r#divisor: i32,
    pub r#per_minute: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct CostTypesRef(pub usize);

impl Deref for CostTypesRef {
    type Target = CostTypesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_CostTypes[self.0]
    }
}

impl CostTypesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static CostTypesRow {
        &TABLE_CostTypes[self.0]
    }
    pub fn get(&self) -> &'static CostTypesRow {
        &TABLE_CostTypes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_CostTypes.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static CostTypesRow)> {
        TABLE_CostTypes
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn print_all_rows() {
        // Print all rows
        for row in TABLE_CostTypes.iter() {
            println!("{:?}", row);
        }
    }
}
