#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ShopItemToStashId: LazyLock<Vec<ShopItemToStashIdRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/shopitemtostashid.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ShopItemToStashIdRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#id2: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#stash_id: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                StashId::from_repr(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ShopItemToStashIdRow {
    pub r#id: String,
    pub r#id2: String,
    pub r#stash_id: Option<StashId>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ShopItemToStashIdRef(pub usize);

impl Deref for ShopItemToStashIdRef {
    type Target = ShopItemToStashIdRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ShopItemToStashId[self.0]
    }
}

impl ShopItemToStashIdRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ShopItemToStashIdRow {
        &TABLE_ShopItemToStashId[self.0]
    }
    pub fn get(&self) -> &'static ShopItemToStashIdRow {
        &TABLE_ShopItemToStashId[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ShopItemToStashId
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ShopItemToStashIdRow)> {
        TABLE_ShopItemToStashId
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
        for row in TABLE_ShopItemToStashId.iter() {
            black_box(row);
        }
    }
}
