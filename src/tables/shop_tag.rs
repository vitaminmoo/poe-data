#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ShopTag: LazyLock<Vec<ShopTagRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/shoptag.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ShopTagRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#is_category: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(16).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#category: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(17..17 + 8).unwrap();
                let value = cell_bytes.get_i64_le();
                ShopTagRef::new(value as usize)
            },
            r#unknown25: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(25).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#sub_flag_art: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(26..26 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#is_guild: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(34).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#parent: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(35..35 + 8).unwrap();
                let value = cell_bytes.get_i64_le();
                ShopTagRef::new(value as usize)
            },
            r#category_art: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(43..43 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ShopTagRow {
    pub r#id: String,
    pub r#name: String,
    pub r#is_category: bool,
    pub r#category: ShopTagRef,
    pub r#unknown25: bool,
    pub r#sub_flag_art: String,
    pub r#is_guild: bool,
    pub r#parent: ShopTagRef,
    pub r#category_art: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ShopTagRef(pub usize);

impl Deref for ShopTagRef {
    type Target = ShopTagRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ShopTag[self.0]
    }
}

impl ShopTagRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ShopTagRow {
        &TABLE_ShopTag[self.0]
    }
    pub fn get(&self) -> &'static ShopTagRow {
        &TABLE_ShopTag[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ShopTag.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ShopTagRow)> {
        TABLE_ShopTag.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn print_all_rows() {
        // Print all rows
        for row in TABLE_ShopTag.iter() {
            println!("{:?}", row);
        }
    }
}
