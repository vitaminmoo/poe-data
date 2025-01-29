#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct HideoutDoodadCategory {
    pub r#id: String,
    pub r#name: String,
}

#[allow(non_upper_case_globals)]
pub static TABLE_HideoutDoodadCategory: LazyLock<Vec<HideoutDoodadCategory>> =
    LazyLock::new(|| {
        RAW_TABLE_HideoutDoodadCategory
            .iter()
            .map(|x| HideoutDoodadCategory {
                r#id: x.r#id.clone(),
                r#name: x.r#name.clone(),
            })
            .collect()
    });

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct HideoutDoodadCategoryRow(pub usize);

impl Deref for HideoutDoodadCategoryRow {
    type Target = HideoutDoodadCategory;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_HideoutDoodadCategory[self.0]
    }
}

impl HideoutDoodadCategoryRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static HideoutDoodadCategory {
        &TABLE_HideoutDoodadCategory[self.0]
    }
    pub fn get(&self) -> &'static HideoutDoodadCategory {
        &TABLE_HideoutDoodadCategory[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_HideoutDoodadCategory
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static HideoutDoodadCategory)> {
        TABLE_HideoutDoodadCategory
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_HideoutDoodadCategory: LazyLock<Vec<HideoutDoodadCategoryRaw>> =
    LazyLock::new(|| {
        const DATA: &str = include_str!("../../data/tables/English/HideoutDoodadCategory.json");
        serde_json::from_str(DATA).unwrap()
    });

#[derive(Debug, Deserialize, Serialize)]
struct HideoutDoodadCategoryRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "Name")]
    pub r#name: String,
}
