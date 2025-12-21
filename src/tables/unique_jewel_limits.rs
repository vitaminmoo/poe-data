#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_UniqueJewelLimits: LazyLock<Vec<UniqueJewelLimitsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/uniquejewellimits.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| UniqueJewelLimitsRow {
            r#jewel_name: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                WordsRef::new(value as usize)
            },
            r#limit: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct UniqueJewelLimitsRow {
    pub r#jewel_name: WordsRef,
    pub r#limit: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct UniqueJewelLimitsRef(pub usize);

impl Deref for UniqueJewelLimitsRef {
    type Target = UniqueJewelLimitsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_UniqueJewelLimits[self.0]
    }
}

impl UniqueJewelLimitsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static UniqueJewelLimitsRow {
        &TABLE_UniqueJewelLimits[self.0]
    }
    pub fn get(&self) -> &'static UniqueJewelLimitsRow {
        &TABLE_UniqueJewelLimits[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_UniqueJewelLimits
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static UniqueJewelLimitsRow)> {
        TABLE_UniqueJewelLimits
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
        for row in TABLE_UniqueJewelLimits.iter() {
            black_box(row);
        }
    }
}
