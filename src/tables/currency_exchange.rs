#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_CurrencyExchange: LazyLock<Vec<CurrencyExchangeRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/currencyexchange.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| CurrencyExchangeRow {
            r#item: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#category: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                CurrencyExchangeCategoriesRef::new(value as usize)
            },
            r#sub_category: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                CurrencyExchangeCategoriesRef::new(value as usize)
            },
            r#unknown48: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(48).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#enabled_in_challenge_league: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(49).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#gold_purchase_fee: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(50..50 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown54: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(54).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct CurrencyExchangeRow {
    pub r#item: BaseItemTypesRef,
    pub r#category: CurrencyExchangeCategoriesRef,
    pub r#sub_category: CurrencyExchangeCategoriesRef,
    pub r#unknown48: bool,
    pub r#enabled_in_challenge_league: bool,
    pub r#gold_purchase_fee: i32,
    pub r#unknown54: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct CurrencyExchangeRef(pub usize);

impl Deref for CurrencyExchangeRef {
    type Target = CurrencyExchangeRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_CurrencyExchange[self.0]
    }
}

impl CurrencyExchangeRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static CurrencyExchangeRow {
        &TABLE_CurrencyExchange[self.0]
    }
    pub fn get(&self) -> &'static CurrencyExchangeRow {
        &TABLE_CurrencyExchange[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_CurrencyExchange.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static CurrencyExchangeRow)> {
        TABLE_CurrencyExchange.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_CurrencyExchange.iter() {
            black_box(row);
        }
    }
}
