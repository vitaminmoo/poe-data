#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct ModType {
    pub r#name: String,
    pub r#mod_sell_price_types_keys: Vec<ModSellPriceTypesRow>,
}

#[allow(non_upper_case_globals)]
pub static TABLE_ModType: LazyLock<Vec<ModType>> = LazyLock::new(|| {
    RAW_TABLE_ModType
        .iter()
        .map(|x| ModType {
            r#name: x.r#name.clone(),
            r#mod_sell_price_types_keys: x
                .r#mod_sell_price_types_keys
                .iter()
                .copied()
                .map(ModSellPriceTypesRow)
                .collect(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ModTypeRow(pub usize);

impl Deref for ModTypeRow {
    type Target = ModType;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_ModType[self.0]
    }
}

impl ModTypeRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ModType {
        &TABLE_ModType[self.0]
    }
    pub fn get(&self) -> &'static ModType {
        &TABLE_ModType[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ModType.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ModType)> {
        TABLE_ModType.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_ModType: LazyLock<Vec<ModTypeRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/ModType.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct ModTypeRaw {
    #[serde(rename = "Name")]
    pub r#name: String,
    #[serde(rename = "ModSellPriceTypesKeys")]
    pub r#mod_sell_price_types_keys: Vec<usize>,
}
