#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MicrotransactionEquippedIconVariations: LazyLock<
    Vec<MicrotransactionEquippedIconVariationsRow>,
> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/microtransactionequippediconvariations.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| MicrotransactionEquippedIconVariationsRow {
            r#unknown0: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#dds_files: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown32: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MicrotransactionEquippedIconVariationsRow {
    pub r#unknown0: i64,
    pub r#dds_files: Vec<String>,
    pub r#unknown32: (usize, usize),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MicrotransactionEquippedIconVariationsRef(pub usize);

impl Deref for MicrotransactionEquippedIconVariationsRef {
    type Target = MicrotransactionEquippedIconVariationsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MicrotransactionEquippedIconVariations[self.0]
    }
}

impl MicrotransactionEquippedIconVariationsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MicrotransactionEquippedIconVariationsRow {
        &TABLE_MicrotransactionEquippedIconVariations[self.0]
    }
    pub fn get(&self) -> &'static MicrotransactionEquippedIconVariationsRow {
        &TABLE_MicrotransactionEquippedIconVariations[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MicrotransactionEquippedIconVariations
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs(
    ) -> impl Iterator<Item = (Self, &'static MicrotransactionEquippedIconVariationsRow)> {
        TABLE_MicrotransactionEquippedIconVariations
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
        for row in TABLE_MicrotransactionEquippedIconVariations.iter() {
            black_box(row);
        }
    }
}
