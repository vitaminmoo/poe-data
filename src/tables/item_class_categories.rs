#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ItemClassCategories: LazyLock<Vec<ItemClassCategoriesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/itemclasscategories.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ItemClassCategoriesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#text: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ItemClassCategoriesRow {
    pub r#id: String,
    pub r#text: String,
    pub r#unknown16: i64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ItemClassCategoriesRef(pub usize);

impl Deref for ItemClassCategoriesRef {
    type Target = ItemClassCategoriesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ItemClassCategories[self.0]
    }
}

impl ItemClassCategoriesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ItemClassCategoriesRow {
        &TABLE_ItemClassCategories[self.0]
    }
    pub fn get(&self) -> &'static ItemClassCategoriesRow {
        &TABLE_ItemClassCategories[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ItemClassCategories
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ItemClassCategoriesRow)> {
        TABLE_ItemClassCategories
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
        for row in TABLE_ItemClassCategories.iter() {
            black_box(row);
        }
    }
}
