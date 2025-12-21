#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_DelveAzuriteShop: LazyLock<Vec<DelveAzuriteShopRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/delveazuriteshop.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| DelveAzuriteShopRow {
            r#base_item_types_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#spawn_weight: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#cost: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#min_depth: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#is_resonator: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(28).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown29: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(29..29 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown33: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(33..33 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct DelveAzuriteShopRow {
    pub r#base_item_types_key: BaseItemTypesRef,
    pub r#spawn_weight: i32,
    pub r#cost: i32,
    pub r#min_depth: i32,
    pub r#is_resonator: bool,
    pub r#unknown29: i32,
    pub r#unknown33: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct DelveAzuriteShopRef(pub usize);

impl Deref for DelveAzuriteShopRef {
    type Target = DelveAzuriteShopRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_DelveAzuriteShop[self.0]
    }
}

impl DelveAzuriteShopRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static DelveAzuriteShopRow {
        &TABLE_DelveAzuriteShop[self.0]
    }
    pub fn get(&self) -> &'static DelveAzuriteShopRow {
        &TABLE_DelveAzuriteShop[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_DelveAzuriteShop
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static DelveAzuriteShopRow)> {
        TABLE_DelveAzuriteShop
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
        for row in TABLE_DelveAzuriteShop.iter() {
            black_box(row);
        }
    }
}
