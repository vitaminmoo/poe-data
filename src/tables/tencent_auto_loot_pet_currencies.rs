#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_TencentAutoLootPetCurrencies: LazyLock<Vec<TencentAutoLootPetCurrenciesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/tencentautolootpetcurrencies.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| TencentAutoLootPetCurrenciesRow {
            r#base_item_types_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#unknown16: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(16).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown17: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(17..17 + 16).unwrap();
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
pub struct TencentAutoLootPetCurrenciesRow {
    pub r#base_item_types_key: BaseItemTypesRef,
    pub r#unknown16: bool,
    pub r#unknown17: (usize, usize),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct TencentAutoLootPetCurrenciesRef(pub usize);

impl Deref for TencentAutoLootPetCurrenciesRef {
    type Target = TencentAutoLootPetCurrenciesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_TencentAutoLootPetCurrencies[self.0]
    }
}

impl TencentAutoLootPetCurrenciesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static TencentAutoLootPetCurrenciesRow {
        &TABLE_TencentAutoLootPetCurrencies[self.0]
    }
    pub fn get(&self) -> &'static TencentAutoLootPetCurrenciesRow {
        &TABLE_TencentAutoLootPetCurrencies[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_TencentAutoLootPetCurrencies.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static TencentAutoLootPetCurrenciesRow)> {
        TABLE_TencentAutoLootPetCurrencies.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_TencentAutoLootPetCurrencies.iter() {
            black_box(row);
        }
    }
}
