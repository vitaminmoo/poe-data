#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_BestiaryRecipes: LazyLock<Vec<BestiaryRecipesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/bestiaryrecipes.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| BestiaryRecipesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#description: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#bestiary_recipe_component_keys: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
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
                    .map(|value| BestiaryRecipeComponentRef::new(value as usize))
                    .collect()
            },
            r#notes: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(32..32 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#category: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BestiaryRecipeCategoriesRef::new(value as usize)
            },
            r#unknown56: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(56).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#achievements: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(57..57 + 16).unwrap();
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
                    .map(|value| AchievementItemsRef::new(value as usize))
                    .collect()
            },
            r#unknown73: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(73).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown74: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(74..74 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown78: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(78..78 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#game_mode: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(82..82 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#flask_mod: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(86..86 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ModsRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct BestiaryRecipesRow {
    pub r#id: String,
    pub r#description: String,
    pub r#bestiary_recipe_component_keys: Vec<BestiaryRecipeComponentRef>,
    pub r#notes: String,
    pub r#category: BestiaryRecipeCategoriesRef,
    pub r#unknown56: bool,
    pub r#achievements: Vec<AchievementItemsRef>,
    pub r#unknown73: bool,
    pub r#unknown74: i32,
    pub r#unknown78: i32,
    pub r#game_mode: i32,
    pub r#flask_mod: ModsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BestiaryRecipesRef(pub usize);

impl Deref for BestiaryRecipesRef {
    type Target = BestiaryRecipesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_BestiaryRecipes[self.0]
    }
}

impl BestiaryRecipesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BestiaryRecipesRow {
        &TABLE_BestiaryRecipes[self.0]
    }
    pub fn get(&self) -> &'static BestiaryRecipesRow {
        &TABLE_BestiaryRecipes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BestiaryRecipes
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BestiaryRecipesRow)> {
        TABLE_BestiaryRecipes
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
        for row in TABLE_BestiaryRecipes.iter() {
            black_box(row);
        }
    }
}
