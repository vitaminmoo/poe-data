#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ExpandingPulse: LazyLock<Vec<ExpandingPulseRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/expandingpulse.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| ExpandingPulseRow {
            r#int_id: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#string_id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(4..4 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown12: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(12..12 + 16).unwrap();
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
            r#unknown28: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(28..28 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 4)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_f32_le())
                    .collect::<Vec<f32>>();
                values
            },
            r#unknown44: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(44..44 + 16).unwrap();
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
            r#unknown60: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(60..60 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown64: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(64..64 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown68: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(68).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ExpandingPulseRow {
    pub r#int_id: i32,
    pub r#string_id: String,
    pub r#unknown12: Vec<i32>,
    pub r#unknown28: Vec<f32>,
    pub r#unknown44: Vec<i64>,
    pub r#unknown60: i32,
    pub r#unknown64: i32,
    pub r#unknown68: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ExpandingPulseRef(pub usize);

impl Deref for ExpandingPulseRef {
    type Target = ExpandingPulseRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ExpandingPulse[self.0]
    }
}

impl ExpandingPulseRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ExpandingPulseRow {
        &TABLE_ExpandingPulse[self.0]
    }
    pub fn get(&self) -> &'static ExpandingPulseRow {
        &TABLE_ExpandingPulse[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ExpandingPulse.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ExpandingPulseRow)> {
        TABLE_ExpandingPulse.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_ExpandingPulse.iter() {
            black_box(row);
        }
    }
}
