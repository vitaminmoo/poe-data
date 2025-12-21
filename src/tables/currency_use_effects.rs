#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_CurrencyUseEffects: LazyLock<Vec<CurrencyUseEffectsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/currencyuseeffects.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| CurrencyUseEffectsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#bk2_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#sound_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown24: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(24).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#bk2_file2: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(25..25 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct CurrencyUseEffectsRow {
    pub r#id: String,
    pub r#bk2_file: String,
    pub r#sound_file: String,
    pub r#unknown24: bool,
    pub r#bk2_file2: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct CurrencyUseEffectsRef(pub usize);

impl Deref for CurrencyUseEffectsRef {
    type Target = CurrencyUseEffectsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_CurrencyUseEffects[self.0]
    }
}

impl CurrencyUseEffectsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static CurrencyUseEffectsRow {
        &TABLE_CurrencyUseEffects[self.0]
    }
    pub fn get(&self) -> &'static CurrencyUseEffectsRow {
        &TABLE_CurrencyUseEffects[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_CurrencyUseEffects
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static CurrencyUseEffectsRow)> {
        TABLE_CurrencyUseEffects
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
        for row in TABLE_CurrencyUseEffects.iter() {
            black_box(row);
        }
    }
}
