#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct ItemClassCategories {
    pub r#id: String,
    pub r#text: String,
}

#[allow(non_upper_case_globals)]
pub static TABLE_ItemClassCategories: LazyLock<Vec<ItemClassCategories>> = LazyLock::new(|| {
    RAW_TABLE_ItemClassCategories
        .iter()
        .map(|x| ItemClassCategories {
            r#id: x.r#id.clone(),
            r#text: x.r#text.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ItemClassCategoriesRow(pub usize);

impl Deref for ItemClassCategoriesRow {
    type Target = ItemClassCategories;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_ItemClassCategories[self.0]
    }
}

impl ItemClassCategoriesRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ItemClassCategories {
        &TABLE_ItemClassCategories[self.0]
    }
    pub fn get(&self) -> &'static ItemClassCategories {
        &TABLE_ItemClassCategories[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ItemClassCategories
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ItemClassCategories)> {
        TABLE_ItemClassCategories
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_ItemClassCategories: LazyLock<Vec<ItemClassCategoriesRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/ItemClassCategories.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct ItemClassCategoriesRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "Text")]
    pub r#text: String,
}
