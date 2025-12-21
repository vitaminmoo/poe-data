#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MicrotransactionConditionalApparitions: LazyLock<
    Vec<MicrotransactionConditionalApparitionsRow>,
> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/microtransactionconditionalapparitions.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| MicrotransactionConditionalApparitionsRow {
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
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown24: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown40: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown56: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(56..56 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown72: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(72..72 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown76: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(76).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown77: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(77..77 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown81: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(81).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MicrotransactionConditionalApparitionsRow {
    pub r#id: String,
    pub r#unknown8: (usize, usize),
    pub r#unknown24: (usize, usize),
    pub r#unknown40: i64,
    pub r#unknown56: i64,
    pub r#unknown72: i32,
    pub r#unknown76: bool,
    pub r#unknown77: i32,
    pub r#unknown81: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MicrotransactionConditionalApparitionsRef(pub usize);

impl Deref for MicrotransactionConditionalApparitionsRef {
    type Target = MicrotransactionConditionalApparitionsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MicrotransactionConditionalApparitions[self.0]
    }
}

impl MicrotransactionConditionalApparitionsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MicrotransactionConditionalApparitionsRow {
        &TABLE_MicrotransactionConditionalApparitions[self.0]
    }
    pub fn get(&self) -> &'static MicrotransactionConditionalApparitionsRow {
        &TABLE_MicrotransactionConditionalApparitions[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MicrotransactionConditionalApparitions
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs(
    ) -> impl Iterator<Item = (Self, &'static MicrotransactionConditionalApparitionsRow)> {
        TABLE_MicrotransactionConditionalApparitions
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
        for row in TABLE_MicrotransactionConditionalApparitions.iter() {
            black_box(row);
        }
    }
}
