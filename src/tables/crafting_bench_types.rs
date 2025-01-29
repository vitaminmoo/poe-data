#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct CraftingBenchTypes {
    pub r#id: String,
}

#[allow(non_upper_case_globals)]
pub static TABLE_CraftingBenchTypes: LazyLock<Vec<CraftingBenchTypes>> = LazyLock::new(|| {
    RAW_TABLE_CraftingBenchTypes
        .iter()
        .map(|x| CraftingBenchTypes {
            r#id: x.r#id.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct CraftingBenchTypesRow(pub usize);

impl Deref for CraftingBenchTypesRow {
    type Target = CraftingBenchTypes;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_CraftingBenchTypes[self.0]
    }
}

impl CraftingBenchTypesRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static CraftingBenchTypes {
        &TABLE_CraftingBenchTypes[self.0]
    }
    pub fn get(&self) -> &'static CraftingBenchTypes {
        &TABLE_CraftingBenchTypes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_CraftingBenchTypes
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static CraftingBenchTypes)> {
        TABLE_CraftingBenchTypes
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_CraftingBenchTypes: LazyLock<Vec<CraftingBenchTypesRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/CraftingBenchTypes.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct CraftingBenchTypesRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
}
