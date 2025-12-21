#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MtxTypeGameSpecific: LazyLock<Vec<MtxTypeGameSpecificRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/mtxtypegamespecific.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| MtxTypeGameSpecificRow {
            r#type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MtxTypesRef::new(value as usize)
            },
            r#hash16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#dds_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(20..20 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#category: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(28..28 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown32: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 4)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i32_le())
                    .collect::<Vec<i32>>();
                values
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MtxTypeGameSpecificRow {
    pub r#type: MtxTypesRef,
    pub r#hash16: i32,
    pub r#dds_file: String,
    pub r#category: i32,
    pub r#unknown32: Vec<i32>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MtxTypeGameSpecificRef(pub usize);

impl Deref for MtxTypeGameSpecificRef {
    type Target = MtxTypeGameSpecificRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MtxTypeGameSpecific[self.0]
    }
}

impl MtxTypeGameSpecificRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MtxTypeGameSpecificRow {
        &TABLE_MtxTypeGameSpecific[self.0]
    }
    pub fn get(&self) -> &'static MtxTypeGameSpecificRow {
        &TABLE_MtxTypeGameSpecific[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MtxTypeGameSpecific
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MtxTypeGameSpecificRow)> {
        TABLE_MtxTypeGameSpecific
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
        for row in TABLE_MtxTypeGameSpecific.iter() {
            println!("{:?}", row);
        }
    }
}
