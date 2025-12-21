#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MicrotransactionAppliedInventoryItemArtVariations: LazyLock<
    Vec<MicrotransactionAppliedInventoryItemArtVariationsRow>,
> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/microtransactionappliedinventoryitemartvariations.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| MicrotransactionAppliedInventoryItemArtVariationsRow {
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
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 4)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i32_le())
                    .collect::<Vec<i32>>();
                values
            },
            r#unknown48: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(48..48 + 16).unwrap();
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
            r#unknown64: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(64..64 + 16).unwrap();
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
        })
        .collect()
});

#[derive(Debug)]
pub struct MicrotransactionAppliedInventoryItemArtVariationsRow {
    pub r#unknown0: i64,
    pub r#dds_files: Vec<String>,
    pub r#unknown32: Vec<i32>,
    pub r#unknown48: Vec<i64>,
    pub r#unknown64: Vec<i64>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MicrotransactionAppliedInventoryItemArtVariationsRef(pub usize);

impl Deref for MicrotransactionAppliedInventoryItemArtVariationsRef {
    type Target = MicrotransactionAppliedInventoryItemArtVariationsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MicrotransactionAppliedInventoryItemArtVariations[self.0]
    }
}

impl MicrotransactionAppliedInventoryItemArtVariationsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MicrotransactionAppliedInventoryItemArtVariationsRow {
        &TABLE_MicrotransactionAppliedInventoryItemArtVariations[self.0]
    }
    pub fn get(&self) -> &'static MicrotransactionAppliedInventoryItemArtVariationsRow {
        &TABLE_MicrotransactionAppliedInventoryItemArtVariations[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MicrotransactionAppliedInventoryItemArtVariations
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<
        Item = (
            Self,
            &'static MicrotransactionAppliedInventoryItemArtVariationsRow,
        ),
    > {
        TABLE_MicrotransactionAppliedInventoryItemArtVariations
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
        for row in TABLE_MicrotransactionAppliedInventoryItemArtVariations.iter() {
            black_box(row);
        }
    }
}
