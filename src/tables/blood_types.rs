#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_BloodTypes: LazyLock<Vec<BloodTypesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/bloodtypes.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| BloodTypesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown8: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
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
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown60: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(60..60 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown64: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(64..64 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown68: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(68..68 + 16).unwrap();
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
            },
            r#unknown84: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(84..84 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown100: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(100..100 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct BloodTypesRow {
    pub r#id: String,
    pub r#unknown8: Vec<i64>,
    pub r#unknown24: i64,
    pub r#unknown40: i64,
    pub r#unknown56: f32,
    pub r#unknown60: f32,
    pub r#unknown64: f32,
    pub r#unknown68: Vec<i64>,
    pub r#unknown84: Vec<String>,
    pub r#unknown100: i64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BloodTypesRef(pub usize);

impl Deref for BloodTypesRef {
    type Target = BloodTypesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_BloodTypes[self.0]
    }
}

impl BloodTypesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BloodTypesRow {
        &TABLE_BloodTypes[self.0]
    }
    pub fn get(&self) -> &'static BloodTypesRow {
        &TABLE_BloodTypes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BloodTypes.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BloodTypesRow)> {
        TABLE_BloodTypes
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
        for row in TABLE_BloodTypes.iter() {
            black_box(row);
        }
    }
}
