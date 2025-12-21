#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_CraftableModTypes: LazyLock<Vec<CraftableModTypesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/craftablemodtypes.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| CraftableModTypesRow {
            r#mod_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ModTypeRef::new(value as usize)
            },
            r#hash16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 2).unwrap();
                let value = cell_bytes.get_u16_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct CraftableModTypesRow {
    pub r#mod_type: ModTypeRef,
    pub r#hash16: u16,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct CraftableModTypesRef(pub usize);

impl Deref for CraftableModTypesRef {
    type Target = CraftableModTypesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_CraftableModTypes[self.0]
    }
}

impl CraftableModTypesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static CraftableModTypesRow {
        &TABLE_CraftableModTypes[self.0]
    }
    pub fn get(&self) -> &'static CraftableModTypesRow {
        &TABLE_CraftableModTypes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_CraftableModTypes
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static CraftableModTypesRow)> {
        TABLE_CraftableModTypes
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
        for row in TABLE_CraftableModTypes.iter() {
            black_box(row);
        }
    }
}
