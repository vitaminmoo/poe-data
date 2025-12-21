#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MonsterCategories: LazyLock<Vec<MonsterCategoriesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/monstercategories.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| MonsterCategoriesRow {
            r#unknown0: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#icon: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(24..24 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MonsterCategoriesRow {
    pub r#unknown0: i64,
    pub r#name: String,
    pub r#icon: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MonsterCategoriesRef(pub usize);

impl Deref for MonsterCategoriesRef {
    type Target = MonsterCategoriesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MonsterCategories[self.0]
    }
}

impl MonsterCategoriesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MonsterCategoriesRow {
        &TABLE_MonsterCategories[self.0]
    }
    pub fn get(&self) -> &'static MonsterCategoriesRow {
        &TABLE_MonsterCategories[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MonsterCategories
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MonsterCategoriesRow)> {
        TABLE_MonsterCategories
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn print_all_rows() {
        // Print all rows
        for row in TABLE_MonsterCategories.iter() {
            println!("{:?}", row);
        }
    }
}
