#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_BlightChestTypes: LazyLock<Vec<BlightChestTypesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/blightchesttypes.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| BlightChestTypesRow {
            r#chests_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ChestsRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct BlightChestTypesRow {
    pub r#chests_key: ChestsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BlightChestTypesRef(pub usize);

impl Deref for BlightChestTypesRef {
    type Target = BlightChestTypesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_BlightChestTypes[self.0]
    }
}

impl BlightChestTypesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BlightChestTypesRow {
        &TABLE_BlightChestTypes[self.0]
    }
    pub fn get(&self) -> &'static BlightChestTypesRow {
        &TABLE_BlightChestTypes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BlightChestTypes
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BlightChestTypesRow)> {
        TABLE_BlightChestTypes
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
        for row in TABLE_BlightChestTypes.iter() {
            black_box(row);
        }
    }
}
