#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct TradeMarketCategory {
    pub r#id: String,
    pub r#name: String,
    pub r#style_flag: MaybeVariant<TradeMarketCategoryStyleFlag>,
    pub r#group: Option<TradeMarketCategoryGroupsRow>,
    pub r#is_disabled: bool,
}

#[allow(non_upper_case_globals)]
pub static TABLE_TradeMarketCategory: LazyLock<Vec<TradeMarketCategory>> = LazyLock::new(|| {
    RAW_TABLE_TradeMarketCategory
        .iter()
        .map(|x| TradeMarketCategory {
            r#id: x.r#id.clone(),
            r#name: x.r#name.clone(),
            r#style_flag: TradeMarketCategoryStyleFlag::from_repr(x.r#style_flag).map_or(
                MaybeVariant::NotVariant(x.r#style_flag),
                MaybeVariant::Variant,
            ),
            r#group: x.r#group.map(TradeMarketCategoryGroupsRow),
            r#is_disabled: x.r#is_disabled.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct TradeMarketCategoryRow(pub usize);

impl Deref for TradeMarketCategoryRow {
    type Target = TradeMarketCategory;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_TradeMarketCategory[self.0]
    }
}

impl TradeMarketCategoryRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static TradeMarketCategory {
        &TABLE_TradeMarketCategory[self.0]
    }
    pub fn get(&self) -> &'static TradeMarketCategory {
        &TABLE_TradeMarketCategory[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_TradeMarketCategory
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static TradeMarketCategory)> {
        TABLE_TradeMarketCategory
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_TradeMarketCategory: LazyLock<Vec<TradeMarketCategoryRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/TradeMarketCategory.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct TradeMarketCategoryRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "Name")]
    pub r#name: String,
    #[serde(rename = "StyleFlag")]
    pub r#style_flag: usize,
    #[serde(rename = "Group")]
    pub r#group: Option<usize>,
    #[serde(rename = "IsDisabled")]
    pub r#is_disabled: bool,
}
