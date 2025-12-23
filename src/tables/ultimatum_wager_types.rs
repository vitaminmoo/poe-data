#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_UltimatumWagerTypes: LazyLock<Vec<UltimatumWagerTypesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/ultimatumwagertypes.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| UltimatumWagerTypesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#hash16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 2).unwrap();
                let value = cell_bytes.get_u16_le();
                value
            },
            r#currency_item: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(10..10 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                CurrencyItemsRef::new(value as usize)
            },
            r#count: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(26..26 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#display_text: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(30..30 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct UltimatumWagerTypesRow {
    pub r#id: String,
    pub r#hash16: u16,
    pub r#currency_item: CurrencyItemsRef,
    pub r#count: i32,
    pub r#display_text: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct UltimatumWagerTypesRef(pub usize);

impl Deref for UltimatumWagerTypesRef {
    type Target = UltimatumWagerTypesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_UltimatumWagerTypes[self.0]
    }
}

impl UltimatumWagerTypesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static UltimatumWagerTypesRow {
        &TABLE_UltimatumWagerTypes[self.0]
    }
    pub fn get(&self) -> &'static UltimatumWagerTypesRow {
        &TABLE_UltimatumWagerTypes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_UltimatumWagerTypes.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static UltimatumWagerTypesRow)> {
        TABLE_UltimatumWagerTypes.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_UltimatumWagerTypes.iter() {
            black_box(row);
        }
    }
}
