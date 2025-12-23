#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MtxTypes: LazyLock<Vec<MtxTypesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/mtxtypes.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| MtxTypesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown8: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(12..12 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown20: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(20..20 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#description1: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(28..28 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#description2: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(36..36 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown44: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(44..44 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#type: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(52..52 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
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
                let mut cell_bytes = row.get(76..76 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown92: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(92..92 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown108: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(108..108 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown116: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(116..116 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown120: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(120..120 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown124: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(124..124 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown128: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(128..128 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown132: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(132..132 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown136: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(136).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown137: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(137..137 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown145: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(145).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown146: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(146).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown147: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(147).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown148: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(148..148 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MtxTypesRow {
    pub r#id: String,
    pub r#unknown8: i32,
    pub r#name: String,
    pub r#unknown20: String,
    pub r#description1: String,
    pub r#description2: String,
    pub r#unknown44: String,
    pub r#type: String,
    pub r#unknown60: i64,
    pub r#unknown76: i64,
    pub r#unknown92: i64,
    pub r#unknown108: String,
    pub r#unknown116: i32,
    pub r#unknown120: i32,
    pub r#unknown124: i32,
    pub r#unknown128: i32,
    pub r#unknown132: i32,
    pub r#unknown136: bool,
    pub r#unknown137: String,
    pub r#unknown145: bool,
    pub r#unknown146: bool,
    pub r#unknown147: bool,
    pub r#unknown148: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MtxTypesRef(pub usize);

impl Deref for MtxTypesRef {
    type Target = MtxTypesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MtxTypes[self.0]
    }
}

impl MtxTypesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MtxTypesRow {
        &TABLE_MtxTypes[self.0]
    }
    pub fn get(&self) -> &'static MtxTypesRow {
        &TABLE_MtxTypes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MtxTypes.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MtxTypesRow)> {
        TABLE_MtxTypes.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_MtxTypes.iter() {
            black_box(row);
        }
    }
}
