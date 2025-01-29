#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct TradeMarketCategoryGroups {
    pub r#id: String,
    pub r#name: String,
}

#[allow(non_upper_case_globals)]
pub static TABLE_TradeMarketCategoryGroups: LazyLock<Vec<TradeMarketCategoryGroups>> =
    LazyLock::new(|| {
        RAW_TABLE_TradeMarketCategoryGroups
            .iter()
            .map(|x| TradeMarketCategoryGroups {
                r#id: x.r#id.clone(),
                r#name: x.r#name.clone(),
            })
            .collect()
    });

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct TradeMarketCategoryGroupsRow(pub usize);

impl Deref for TradeMarketCategoryGroupsRow {
    type Target = TradeMarketCategoryGroups;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_TradeMarketCategoryGroups[self.0]
    }
}

impl TradeMarketCategoryGroupsRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static TradeMarketCategoryGroups {
        &TABLE_TradeMarketCategoryGroups[self.0]
    }
    pub fn get(&self) -> &'static TradeMarketCategoryGroups {
        &TABLE_TradeMarketCategoryGroups[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_TradeMarketCategoryGroups
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static TradeMarketCategoryGroups)> {
        TABLE_TradeMarketCategoryGroups
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_TradeMarketCategoryGroups: LazyLock<Vec<TradeMarketCategoryGroupsRaw>> =
    LazyLock::new(|| {
        const DATA: &str = include_str!("../../data/tables/English/TradeMarketCategoryGroups.json");
        serde_json::from_str(DATA).unwrap()
    });

#[derive(Debug, Deserialize, Serialize)]
struct TradeMarketCategoryGroupsRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "Name")]
    pub r#name: String,
}
