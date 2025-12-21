#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_RecipeUnlockDisplay: LazyLock<Vec<RecipeUnlockDisplayRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/recipeunlockdisplay.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| RecipeUnlockDisplayRow {
            r#recipe_id: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#description: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(4..4 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#crafting_item_class_categories_keys: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(12..12 + 16).unwrap();
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
            r#unlock_description: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(28..28 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#rank: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(36..36 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unlock_area: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                WorldAreasRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct RecipeUnlockDisplayRow {
    pub r#recipe_id: i32,
    pub r#description: String,
    pub r#crafting_item_class_categories_keys: Vec<CraftingItemClassCategoriesRef>,
    pub r#unlock_description: String,
    pub r#rank: i32,
    pub r#unlock_area: WorldAreasRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct RecipeUnlockDisplayRef(pub usize);

impl Deref for RecipeUnlockDisplayRef {
    type Target = RecipeUnlockDisplayRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_RecipeUnlockDisplay[self.0]
    }
}

impl RecipeUnlockDisplayRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static RecipeUnlockDisplayRow {
        &TABLE_RecipeUnlockDisplay[self.0]
    }
    pub fn get(&self) -> &'static RecipeUnlockDisplayRow {
        &TABLE_RecipeUnlockDisplay[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_RecipeUnlockDisplay
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static RecipeUnlockDisplayRow)> {
        TABLE_RecipeUnlockDisplay
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
        for row in TABLE_RecipeUnlockDisplay.iter() {
            black_box(row);
        }
    }
}
