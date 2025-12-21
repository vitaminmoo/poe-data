#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_BlightCraftingTypes: LazyLock<Vec<BlightCraftingTypesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/blightcraftingtypes.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| BlightCraftingTypesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown8: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown12: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(12).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct BlightCraftingTypesRow {
    pub r#id: String,
    pub r#unknown8: i32,
    pub r#unknown12: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BlightCraftingTypesRef(pub usize);

impl Deref for BlightCraftingTypesRef {
    type Target = BlightCraftingTypesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_BlightCraftingTypes[self.0]
    }
}

impl BlightCraftingTypesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BlightCraftingTypesRow {
        &TABLE_BlightCraftingTypes[self.0]
    }
    pub fn get(&self) -> &'static BlightCraftingTypesRow {
        &TABLE_BlightCraftingTypes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BlightCraftingTypes
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BlightCraftingTypesRow)> {
        TABLE_BlightCraftingTypes
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
        for row in TABLE_BlightCraftingTypes.iter() {
            black_box(row);
        }
    }
}
