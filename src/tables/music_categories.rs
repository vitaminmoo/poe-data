#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MusicCategories: LazyLock<Vec<MusicCategoriesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/musiccategories.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| MusicCategoriesRow {
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
            r#order: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown20: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(20).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MusicCategoriesRow {
    pub r#id: String,
    pub r#name: String,
    pub r#order: i32,
    pub r#unknown20: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MusicCategoriesRef(pub usize);

impl Deref for MusicCategoriesRef {
    type Target = MusicCategoriesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MusicCategories[self.0]
    }
}

impl MusicCategoriesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MusicCategoriesRow {
        &TABLE_MusicCategories[self.0]
    }
    pub fn get(&self) -> &'static MusicCategoriesRow {
        &TABLE_MusicCategories[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MusicCategories.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MusicCategoriesRow)> {
        TABLE_MusicCategories.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_MusicCategories.iter() {
            black_box(row);
        }
    }
}
