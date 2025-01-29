#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct CraftingItemClassCategories {
    pub r#id: String,
    pub r#item_classes: Vec<ItemClassesRow>,
    pub r#text: String,
}

#[allow(non_upper_case_globals)]
pub static TABLE_CraftingItemClassCategories: LazyLock<Vec<CraftingItemClassCategories>> =
    LazyLock::new(|| {
        RAW_TABLE_CraftingItemClassCategories
            .iter()
            .map(|x| CraftingItemClassCategories {
                r#id: x.r#id.clone(),
                r#item_classes: x
                    .r#item_classes
                    .iter()
                    .copied()
                    .map(ItemClassesRow)
                    .collect(),
                r#text: x.r#text.clone(),
            })
            .collect()
    });

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct CraftingItemClassCategoriesRow(pub usize);

impl Deref for CraftingItemClassCategoriesRow {
    type Target = CraftingItemClassCategories;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_CraftingItemClassCategories[self.0]
    }
}

impl CraftingItemClassCategoriesRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static CraftingItemClassCategories {
        &TABLE_CraftingItemClassCategories[self.0]
    }
    pub fn get(&self) -> &'static CraftingItemClassCategories {
        &TABLE_CraftingItemClassCategories[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_CraftingItemClassCategories
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static CraftingItemClassCategories)> {
        TABLE_CraftingItemClassCategories
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_CraftingItemClassCategories: LazyLock<Vec<CraftingItemClassCategoriesRaw>> =
    LazyLock::new(|| {
        const DATA: &str =
            include_str!("../../data/tables/English/CraftingItemClassCategories.json");
        serde_json::from_str(DATA).unwrap()
    });

#[derive(Debug, Deserialize, Serialize)]
struct CraftingItemClassCategoriesRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "ItemClasses")]
    pub r#item_classes: Vec<usize>,
    #[serde(rename = "Text")]
    pub r#text: String,
}
