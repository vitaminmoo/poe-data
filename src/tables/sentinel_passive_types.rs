#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_SentinelPassiveTypes: LazyLock<Vec<SentinelPassiveTypesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/sentinelpassivetypes.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| SentinelPassiveTypesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#default_icon: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#active_icon: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#drone_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                DroneTypesRef::new(value as usize)
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
pub struct SentinelPassiveTypesRow {
    pub r#id: String,
    pub r#default_icon: String,
    pub r#active_icon: String,
    pub r#drone_type: DroneTypesRef,
    pub r#unknown40: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct SentinelPassiveTypesRef(pub usize);

impl Deref for SentinelPassiveTypesRef {
    type Target = SentinelPassiveTypesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_SentinelPassiveTypes[self.0]
    }
}

impl SentinelPassiveTypesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static SentinelPassiveTypesRow {
        &TABLE_SentinelPassiveTypes[self.0]
    }
    pub fn get(&self) -> &'static SentinelPassiveTypesRow {
        &TABLE_SentinelPassiveTypes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_SentinelPassiveTypes.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static SentinelPassiveTypesRow)> {
        TABLE_SentinelPassiveTypes.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_SentinelPassiveTypes.iter() {
            black_box(row);
        }
    }
}
