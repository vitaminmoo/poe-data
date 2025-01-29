#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct CostTypes {
    pub r#id: String,
    pub r#stats_key: Option<StatsRow>,
    pub r#format_text: String,
}

#[allow(non_upper_case_globals)]
pub static TABLE_CostTypes: LazyLock<Vec<CostTypes>> = LazyLock::new(|| {
    RAW_TABLE_CostTypes
        .iter()
        .map(|x| CostTypes {
            r#id: x.r#id.clone(),
            r#stats_key: x.r#stats_key.map(StatsRow),
            r#format_text: x.r#format_text.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct CostTypesRow(pub usize);

impl Deref for CostTypesRow {
    type Target = CostTypes;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_CostTypes[self.0]
    }
}

impl CostTypesRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static CostTypes {
        &TABLE_CostTypes[self.0]
    }
    pub fn get(&self) -> &'static CostTypes {
        &TABLE_CostTypes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_CostTypes.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static CostTypes)> {
        TABLE_CostTypes
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_CostTypes: LazyLock<Vec<CostTypesRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/CostTypes.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct CostTypesRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "StatsKey")]
    pub r#stats_key: Option<usize>,
    #[serde(rename = "FormatText")]
    pub r#format_text: String,
}
