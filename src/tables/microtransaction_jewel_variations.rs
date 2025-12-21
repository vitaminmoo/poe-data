#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MicrotransactionJewelVariations: LazyLock<
    Vec<MicrotransactionJewelVariationsRow>,
> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/microtransactionjewelvariations.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| MicrotransactionJewelVariationsRow {
            r#mtx_type_game_specific: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MtxTypeGameSpecificRef::new(value as usize)
            },
            r#unknown16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown20: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(20..20 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown28: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(28..28 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown36: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(36..36 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown40: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(40..40 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown48: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(48..48 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MicrotransactionJewelVariationsRow {
    pub r#mtx_type_game_specific: MtxTypeGameSpecificRef,
    pub r#unknown16: i32,
    pub r#unknown20: String,
    pub r#unknown28: String,
    pub r#unknown36: i32,
    pub r#unknown40: String,
    pub r#unknown48: i64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MicrotransactionJewelVariationsRef(pub usize);

impl Deref for MicrotransactionJewelVariationsRef {
    type Target = MicrotransactionJewelVariationsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MicrotransactionJewelVariations[self.0]
    }
}

impl MicrotransactionJewelVariationsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MicrotransactionJewelVariationsRow {
        &TABLE_MicrotransactionJewelVariations[self.0]
    }
    pub fn get(&self) -> &'static MicrotransactionJewelVariationsRow {
        &TABLE_MicrotransactionJewelVariations[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MicrotransactionJewelVariations
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs(
    ) -> impl Iterator<Item = (Self, &'static MicrotransactionJewelVariationsRow)> {
        TABLE_MicrotransactionJewelVariations
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
        for row in TABLE_MicrotransactionJewelVariations.iter() {
            black_box(row);
        }
    }
}
