#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct CraftingBenchUnlockCategories {
    pub r#id: String,
    pub r#unlock_type: String,
    pub r#crafting_item_class_categories: Vec<CraftingItemClassCategoriesRow>,
    pub r#obtaining_description: String,
}

#[allow(non_upper_case_globals)]
pub static TABLE_CraftingBenchUnlockCategories: LazyLock<Vec<CraftingBenchUnlockCategories>> =
    LazyLock::new(|| {
        RAW_TABLE_CraftingBenchUnlockCategories
            .iter()
            .map(|x| CraftingBenchUnlockCategories {
                r#id: x.r#id.clone(),
                r#unlock_type: x.r#unlock_type.clone(),
                r#crafting_item_class_categories: x
                    .r#crafting_item_class_categories
                    .iter()
                    .copied()
                    .map(CraftingItemClassCategoriesRow)
                    .collect(),
                r#obtaining_description: x.r#obtaining_description.clone(),
            })
            .collect()
    });

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct CraftingBenchUnlockCategoriesRow(pub usize);

impl Deref for CraftingBenchUnlockCategoriesRow {
    type Target = CraftingBenchUnlockCategories;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_CraftingBenchUnlockCategories[self.0]
    }
}

impl CraftingBenchUnlockCategoriesRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static CraftingBenchUnlockCategories {
        &TABLE_CraftingBenchUnlockCategories[self.0]
    }
    pub fn get(&self) -> &'static CraftingBenchUnlockCategories {
        &TABLE_CraftingBenchUnlockCategories[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_CraftingBenchUnlockCategories
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static CraftingBenchUnlockCategories)>
    {
        TABLE_CraftingBenchUnlockCategories
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_CraftingBenchUnlockCategories: LazyLock<Vec<CraftingBenchUnlockCategoriesRaw>> =
    LazyLock::new(|| {
        const DATA: &str =
            include_str!("../../data/tables/English/CraftingBenchUnlockCategories.json");
        serde_json::from_str(DATA).unwrap()
    });

#[derive(Debug, Deserialize, Serialize)]
struct CraftingBenchUnlockCategoriesRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "UnlockType")]
    pub r#unlock_type: String,
    #[serde(rename = "CraftingItemClassCategories")]
    pub r#crafting_item_class_categories: Vec<usize>,
    #[serde(rename = "ObtainingDescription")]
    pub r#obtaining_description: String,
}
