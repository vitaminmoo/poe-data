#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MicrotransactionSocialFrameVariations: LazyLock<Vec<MicrotransactionSocialFrameVariationsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/microtransactionsocialframevariations.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| MicrotransactionSocialFrameVariationsRow {
            r#mtx_type_game_specific: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MtxTypeGameSpecificRef::new(value as usize)
            },
            r#id: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 2).unwrap();
                let value = cell_bytes.get_i16_le();
                value
            },
            r#bk2_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(18..18 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MicrotransactionSocialFrameVariationsRow {
    pub r#mtx_type_game_specific: MtxTypeGameSpecificRef,
    pub r#id: i16,
    pub r#bk2_file: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MicrotransactionSocialFrameVariationsRef(pub usize);

impl Deref for MicrotransactionSocialFrameVariationsRef {
    type Target = MicrotransactionSocialFrameVariationsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MicrotransactionSocialFrameVariations[self.0]
    }
}

impl MicrotransactionSocialFrameVariationsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MicrotransactionSocialFrameVariationsRow {
        &TABLE_MicrotransactionSocialFrameVariations[self.0]
    }
    pub fn get(&self) -> &'static MicrotransactionSocialFrameVariationsRow {
        &TABLE_MicrotransactionSocialFrameVariations[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MicrotransactionSocialFrameVariations.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MicrotransactionSocialFrameVariationsRow)> {
        TABLE_MicrotransactionSocialFrameVariations.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_MicrotransactionSocialFrameVariations.iter() {
            black_box(row);
        }
    }
}
