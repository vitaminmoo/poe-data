#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_PassiveJewelArt: LazyLock<Vec<PassiveJewelArtRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/passivejewelart.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| PassiveJewelArtRow {
            r#base_item_types_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#socketed_image: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#socketed_image_large: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(24..24 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown32: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct PassiveJewelArtRow {
    pub r#base_item_types_key: BaseItemTypesRef,
    pub r#socketed_image: String,
    pub r#socketed_image_large: String,
    pub r#unknown32: i64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct PassiveJewelArtRef(pub usize);

impl Deref for PassiveJewelArtRef {
    type Target = PassiveJewelArtRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_PassiveJewelArt[self.0]
    }
}

impl PassiveJewelArtRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static PassiveJewelArtRow {
        &TABLE_PassiveJewelArt[self.0]
    }
    pub fn get(&self) -> &'static PassiveJewelArtRow {
        &TABLE_PassiveJewelArt[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_PassiveJewelArt
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static PassiveJewelArtRow)> {
        TABLE_PassiveJewelArt
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
        for row in TABLE_PassiveJewelArt.iter() {
            black_box(row);
        }
    }
}
