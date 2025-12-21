#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_CraftingBenchUnlockCategories: LazyLock<Vec<CraftingBenchUnlockCategoriesRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/craftingbenchunlockcategories.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| CraftingBenchUnlockCategoriesRow {
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
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(12..12 + 16).unwrap();
                    let count = cell_bytes.get_u64_le() as usize;
                    let offset = cell_bytes.get_u64_le() as usize;
                    // array_mutator column.array == true && column.type else
                    let values = df
                        .array_from_offset(offset, count, 4)
                        .unwrap()
                        .iter()
                        .map(|x| x.clone().get_i32_le())
                        .collect::<Vec<i32>>();
                    values
                },
                r#unlock_type: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(28..28 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#crafting_item_class_categories: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(36..36 + 16).unwrap();
                    let count = cell_bytes.get_u64_le() as usize;
                    let offset = cell_bytes.get_u64_le() as usize;
                    // array_mutator column.array == true && column.type else
                    let values = df
                        .array_from_offset(offset, count, 16)
                        .unwrap()
                        .iter()
                        .map(|x| x.clone().get_i64_le())
                        .collect::<Vec<i64>>();
                    values
                        .into_iter()
                        .map(|value| CraftingItemClassCategoriesRef::new(value as usize))
                        .collect()
                },
                r#obtaining_description: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(52..52 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct CraftingBenchUnlockCategoriesRow {
    pub r#id: String,
    pub r#unknown8: i32,
    pub r#unknown12: Vec<i32>,
    pub r#unlock_type: String,
    pub r#crafting_item_class_categories: Vec<CraftingItemClassCategoriesRef>,
    pub r#obtaining_description: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct CraftingBenchUnlockCategoriesRef(pub usize);

impl Deref for CraftingBenchUnlockCategoriesRef {
    type Target = CraftingBenchUnlockCategoriesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_CraftingBenchUnlockCategories[self.0]
    }
}

impl CraftingBenchUnlockCategoriesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static CraftingBenchUnlockCategoriesRow {
        &TABLE_CraftingBenchUnlockCategories[self.0]
    }
    pub fn get(&self) -> &'static CraftingBenchUnlockCategoriesRow {
        &TABLE_CraftingBenchUnlockCategories[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_CraftingBenchUnlockCategories
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs(
    ) -> impl Iterator<Item = (Self, &'static CraftingBenchUnlockCategoriesRow)> {
        TABLE_CraftingBenchUnlockCategories
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
        for row in TABLE_CraftingBenchUnlockCategories.iter() {
            black_box(row);
        }
    }
}
