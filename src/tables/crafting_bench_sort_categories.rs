#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct CraftingBenchSortCategories {
    pub r#id: String,
    pub r#name: Option<ClientStringsRow>,
    pub r#is_visible: bool,
    pub r#type: Option<CraftingBenchTypesRow>,
}

#[allow(non_upper_case_globals)]
pub static TABLE_CraftingBenchSortCategories: LazyLock<Vec<CraftingBenchSortCategories>> =
    LazyLock::new(|| {
        RAW_TABLE_CraftingBenchSortCategories
            .iter()
            .map(|x| CraftingBenchSortCategories {
                r#id: x.r#id.clone(),
                r#name: x.r#name.map(ClientStringsRow),
                r#is_visible: x.r#is_visible.clone(),
                r#type: x.r#type.map(CraftingBenchTypesRow),
            })
            .collect()
    });

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct CraftingBenchSortCategoriesRow(pub usize);

impl Deref for CraftingBenchSortCategoriesRow {
    type Target = CraftingBenchSortCategories;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_CraftingBenchSortCategories[self.0]
    }
}

impl CraftingBenchSortCategoriesRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static CraftingBenchSortCategories {
        &TABLE_CraftingBenchSortCategories[self.0]
    }
    pub fn get(&self) -> &'static CraftingBenchSortCategories {
        &TABLE_CraftingBenchSortCategories[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_CraftingBenchSortCategories
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static CraftingBenchSortCategories)> {
        TABLE_CraftingBenchSortCategories
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_CraftingBenchSortCategories: LazyLock<Vec<CraftingBenchSortCategoriesRaw>> =
    LazyLock::new(|| {
        const DATA: &str =
            include_str!("../../data/tables/English/CraftingBenchSortCategories.json");
        serde_json::from_str(DATA).unwrap()
    });

#[derive(Debug, Deserialize, Serialize)]
struct CraftingBenchSortCategoriesRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "Name")]
    pub r#name: Option<usize>,
    #[serde(rename = "IsVisible")]
    pub r#is_visible: bool,
    #[serde(rename = "Type")]
    pub r#type: Option<usize>,
}
