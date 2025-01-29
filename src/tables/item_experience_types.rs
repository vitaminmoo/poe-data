#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct ItemExperienceTypes {
    pub r#id: String,
}

#[allow(non_upper_case_globals)]
pub static TABLE_ItemExperienceTypes: LazyLock<Vec<ItemExperienceTypes>> = LazyLock::new(|| {
    RAW_TABLE_ItemExperienceTypes
        .iter()
        .map(|x| ItemExperienceTypes {
            r#id: x.r#id.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ItemExperienceTypesRow(pub usize);

impl Deref for ItemExperienceTypesRow {
    type Target = ItemExperienceTypes;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_ItemExperienceTypes[self.0]
    }
}

impl ItemExperienceTypesRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ItemExperienceTypes {
        &TABLE_ItemExperienceTypes[self.0]
    }
    pub fn get(&self) -> &'static ItemExperienceTypes {
        &TABLE_ItemExperienceTypes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ItemExperienceTypes
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ItemExperienceTypes)> {
        TABLE_ItemExperienceTypes
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_ItemExperienceTypes: LazyLock<Vec<ItemExperienceTypesRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/ItemExperienceTypes.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct ItemExperienceTypesRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
}
