#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct RecipeUnlockDisplay {
    pub r#recipe_id: i32,
    pub r#description: String,
    pub r#crafting_item_class_categories_keys: Vec<CraftingItemClassCategoriesRow>,
    pub r#unlock_description: String,
    pub r#rank: i32,
    pub r#unlock_area: Option<WorldAreasRow>,
}

#[allow(non_upper_case_globals)]
pub static TABLE_RecipeUnlockDisplay: LazyLock<Vec<RecipeUnlockDisplay>> = LazyLock::new(|| {
    RAW_TABLE_RecipeUnlockDisplay
        .iter()
        .map(|x| RecipeUnlockDisplay {
            r#recipe_id: x.r#recipe_id.clone(),
            r#description: x.r#description.clone(),
            r#crafting_item_class_categories_keys: x
                .r#crafting_item_class_categories_keys
                .iter()
                .copied()
                .map(CraftingItemClassCategoriesRow)
                .collect(),
            r#unlock_description: x.r#unlock_description.clone(),
            r#rank: x.r#rank.clone(),
            r#unlock_area: x.r#unlock_area.map(WorldAreasRow),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct RecipeUnlockDisplayRow(pub usize);

impl Deref for RecipeUnlockDisplayRow {
    type Target = RecipeUnlockDisplay;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_RecipeUnlockDisplay[self.0]
    }
}

impl RecipeUnlockDisplayRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static RecipeUnlockDisplay {
        &TABLE_RecipeUnlockDisplay[self.0]
    }
    pub fn get(&self) -> &'static RecipeUnlockDisplay {
        &TABLE_RecipeUnlockDisplay[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_RecipeUnlockDisplay
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static RecipeUnlockDisplay)> {
        TABLE_RecipeUnlockDisplay
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_RecipeUnlockDisplay: LazyLock<Vec<RecipeUnlockDisplayRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/RecipeUnlockDisplay.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct RecipeUnlockDisplayRaw {
    #[serde(rename = "RecipeId")]
    pub r#recipe_id: i32,
    #[serde(rename = "Description")]
    pub r#description: String,
    #[serde(rename = "CraftingItemClassCategoriesKeys")]
    pub r#crafting_item_class_categories_keys: Vec<usize>,
    #[serde(rename = "UnlockDescription")]
    pub r#unlock_description: String,
    #[serde(rename = "Rank")]
    pub r#rank: i32,
    #[serde(rename = "UnlockArea")]
    pub r#unlock_area: Option<usize>,
}
