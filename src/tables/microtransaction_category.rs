#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MicrotransactionCategory: LazyLock<Vec<MicrotransactionCategoryRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/microtransactioncategory.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| MicrotransactionCategoryRow {
                r#id: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(0..0 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#name: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(4..4 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#unknown12: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(12..12 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct MicrotransactionCategoryRow {
    pub r#id: i32,
    pub r#name: String,
    pub r#unknown12: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MicrotransactionCategoryRef(pub usize);

impl Deref for MicrotransactionCategoryRef {
    type Target = MicrotransactionCategoryRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MicrotransactionCategory[self.0]
    }
}

impl MicrotransactionCategoryRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MicrotransactionCategoryRow {
        &TABLE_MicrotransactionCategory[self.0]
    }
    pub fn get(&self) -> &'static MicrotransactionCategoryRow {
        &TABLE_MicrotransactionCategory[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MicrotransactionCategory
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MicrotransactionCategoryRow)> {
        TABLE_MicrotransactionCategory
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
        for row in TABLE_MicrotransactionCategory.iter() {
            black_box(row);
        }
    }
}
