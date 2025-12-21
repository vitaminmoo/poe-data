#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_TieredCurrency: LazyLock<Vec<TieredCurrencyRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/tieredcurrency.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| TieredCurrencyRow {
            r#base_item_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#tier: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#minimum_mod_level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#lower_tier_base_item_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct TieredCurrencyRow {
    pub r#base_item_type: BaseItemTypesRef,
    pub r#tier: i32,
    pub r#minimum_mod_level: i32,
    pub r#lower_tier_base_item_type: BaseItemTypesRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct TieredCurrencyRef(pub usize);

impl Deref for TieredCurrencyRef {
    type Target = TieredCurrencyRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_TieredCurrency[self.0]
    }
}

impl TieredCurrencyRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static TieredCurrencyRow {
        &TABLE_TieredCurrency[self.0]
    }
    pub fn get(&self) -> &'static TieredCurrencyRow {
        &TABLE_TieredCurrency[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_TieredCurrency
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static TieredCurrencyRow)> {
        TABLE_TieredCurrency
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
        for row in TABLE_TieredCurrency.iter() {
            black_box(row);
        }
    }
}
