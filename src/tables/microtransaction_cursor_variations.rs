#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MicrotransactionCursorVariations: LazyLock<
    Vec<MicrotransactionCursorVariationsRow>,
> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/microtransactioncursorvariations.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| MicrotransactionCursorVariationsRow {
            r#mtx_type_game_specific: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MtxTypeGameSpecificRef::new(value as usize)
            },
            r#unknown16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown32: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 2).unwrap();
                let value = cell_bytes.get_i16_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MicrotransactionCursorVariationsRow {
    pub r#mtx_type_game_specific: MtxTypeGameSpecificRef,
    pub r#unknown16: i64,
    pub r#unknown32: i16,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MicrotransactionCursorVariationsRef(pub usize);

impl Deref for MicrotransactionCursorVariationsRef {
    type Target = MicrotransactionCursorVariationsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MicrotransactionCursorVariations[self.0]
    }
}

impl MicrotransactionCursorVariationsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MicrotransactionCursorVariationsRow {
        &TABLE_MicrotransactionCursorVariations[self.0]
    }
    pub fn get(&self) -> &'static MicrotransactionCursorVariationsRow {
        &TABLE_MicrotransactionCursorVariations[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MicrotransactionCursorVariations
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs(
    ) -> impl Iterator<Item = (Self, &'static MicrotransactionCursorVariationsRow)> {
        TABLE_MicrotransactionCursorVariations
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
        for row in TABLE_MicrotransactionCursorVariations.iter() {
            black_box(row);
        }
    }
}
