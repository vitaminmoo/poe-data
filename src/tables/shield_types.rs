#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ShieldTypes: LazyLock<Vec<ShieldTypesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/shieldtypes.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ShieldTypesRow {
            r#base_item_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#block: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ShieldTypesRow {
    pub r#base_item_type: BaseItemTypesRef,
    pub r#block: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ShieldTypesRef(pub usize);

impl Deref for ShieldTypesRef {
    type Target = ShieldTypesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ShieldTypes[self.0]
    }
}

impl ShieldTypesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ShieldTypesRow {
        &TABLE_ShieldTypes[self.0]
    }
    pub fn get(&self) -> &'static ShieldTypesRow {
        &TABLE_ShieldTypes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ShieldTypes.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ShieldTypesRow)> {
        TABLE_ShieldTypes
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
        for row in TABLE_ShieldTypes.iter() {
            black_box(row);
        }
    }
}
