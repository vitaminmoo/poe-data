#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct ModSellPriceTypes {
    pub r#id: String,
}

#[allow(non_upper_case_globals)]
pub static TABLE_ModSellPriceTypes: LazyLock<Vec<ModSellPriceTypes>> = LazyLock::new(|| {
    RAW_TABLE_ModSellPriceTypes
        .iter()
        .map(|x| ModSellPriceTypes {
            r#id: x.r#id.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ModSellPriceTypesRow(pub usize);

impl Deref for ModSellPriceTypesRow {
    type Target = ModSellPriceTypes;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_ModSellPriceTypes[self.0]
    }
}

impl ModSellPriceTypesRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ModSellPriceTypes {
        &TABLE_ModSellPriceTypes[self.0]
    }
    pub fn get(&self) -> &'static ModSellPriceTypes {
        &TABLE_ModSellPriceTypes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ModSellPriceTypes
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ModSellPriceTypes)> {
        TABLE_ModSellPriceTypes
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_ModSellPriceTypes: LazyLock<Vec<ModSellPriceTypesRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/ModSellPriceTypes.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct ModSellPriceTypesRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
}
