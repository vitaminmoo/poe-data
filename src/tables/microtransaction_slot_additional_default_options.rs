#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MicrotransactionSlotAdditionalDefaultOptions: LazyLock<
    Vec<MicrotransactionSlotAdditionalDefaultOptionsRow>,
> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/microtransactionslotadditionaldefaultoptions.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| MicrotransactionSlotAdditionalDefaultOptionsRow {
            r#unknown0: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown4: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(4..4 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown8: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown16: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MicrotransactionSlotAdditionalDefaultOptionsRow {
    pub r#unknown0: i32,
    pub r#unknown4: i32,
    pub r#unknown8: String,
    pub r#unknown16: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MicrotransactionSlotAdditionalDefaultOptionsRef(pub usize);

impl Deref for MicrotransactionSlotAdditionalDefaultOptionsRef {
    type Target = MicrotransactionSlotAdditionalDefaultOptionsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MicrotransactionSlotAdditionalDefaultOptions[self.0]
    }
}

impl MicrotransactionSlotAdditionalDefaultOptionsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MicrotransactionSlotAdditionalDefaultOptionsRow {
        &TABLE_MicrotransactionSlotAdditionalDefaultOptions[self.0]
    }
    pub fn get(&self) -> &'static MicrotransactionSlotAdditionalDefaultOptionsRow {
        &TABLE_MicrotransactionSlotAdditionalDefaultOptions[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MicrotransactionSlotAdditionalDefaultOptions
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<
        Item = (
            Self,
            &'static MicrotransactionSlotAdditionalDefaultOptionsRow,
        ),
    > {
        TABLE_MicrotransactionSlotAdditionalDefaultOptions
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
        for row in TABLE_MicrotransactionSlotAdditionalDefaultOptions.iter() {
            black_box(row);
        }
    }
}
