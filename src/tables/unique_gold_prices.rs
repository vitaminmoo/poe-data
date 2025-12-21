#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_UniqueGoldPrices: LazyLock<Vec<UniqueGoldPricesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/uniquegoldprices.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| UniqueGoldPricesRow {
            r#name: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                WordsRef::new(value as usize)
            },
            r#price: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct UniqueGoldPricesRow {
    pub r#name: WordsRef,
    pub r#price: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct UniqueGoldPricesRef(pub usize);

impl Deref for UniqueGoldPricesRef {
    type Target = UniqueGoldPricesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_UniqueGoldPrices[self.0]
    }
}

impl UniqueGoldPricesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static UniqueGoldPricesRow {
        &TABLE_UniqueGoldPrices[self.0]
    }
    pub fn get(&self) -> &'static UniqueGoldPricesRow {
        &TABLE_UniqueGoldPrices[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_UniqueGoldPrices
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static UniqueGoldPricesRow)> {
        TABLE_UniqueGoldPrices
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
        for row in TABLE_UniqueGoldPrices.iter() {
            black_box(row);
        }
    }
}
