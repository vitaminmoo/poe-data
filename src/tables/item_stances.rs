#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct ItemStances {
    pub r#id: String,
}

#[allow(non_upper_case_globals)]
pub static TABLE_ItemStances: LazyLock<Vec<ItemStances>> = LazyLock::new(|| {
    RAW_TABLE_ItemStances
        .iter()
        .map(|x| ItemStances {
            r#id: x.r#id.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ItemStancesRow(pub usize);

impl Deref for ItemStancesRow {
    type Target = ItemStances;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_ItemStances[self.0]
    }
}

impl ItemStancesRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ItemStances {
        &TABLE_ItemStances[self.0]
    }
    pub fn get(&self) -> &'static ItemStances {
        &TABLE_ItemStances[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ItemStances.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ItemStances)> {
        TABLE_ItemStances
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_ItemStances: LazyLock<Vec<ItemStancesRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/ItemStances.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct ItemStancesRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
}
