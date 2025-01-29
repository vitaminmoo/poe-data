#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct BuffDefinitions {
    pub r#id: String,
    pub r#description: String,
    pub r#invisible: bool,
    pub r#removable: bool,
    pub r#name: String,
    pub r#stats_keys: Vec<StatsRow>,
    pub r#maximum_stats_key: Option<StatsRow>,
    pub r#current_stats_key: Option<StatsRow>,
    pub r#buff_visuals_key: Option<BuffVisualsRow>,
    pub r#buff_category: i32,
    pub r#buff_limit: i32,
    pub r#id2: String,
    pub r#is_recovery: bool,
    pub r#binary_stats: Vec<StatsRow>,
}

#[allow(non_upper_case_globals)]
pub static TABLE_BuffDefinitions: LazyLock<Vec<BuffDefinitions>> = LazyLock::new(|| {
    RAW_TABLE_BuffDefinitions
        .iter()
        .map(|x| BuffDefinitions {
            r#id: x.r#id.clone(),
            r#description: x.r#description.clone(),
            r#invisible: x.r#invisible.clone(),
            r#removable: x.r#removable.clone(),
            r#name: x.r#name.clone(),
            r#stats_keys: x.r#stats_keys.iter().copied().map(StatsRow).collect(),
            r#maximum_stats_key: x.r#maximum_stats_key.map(StatsRow),
            r#current_stats_key: x.r#current_stats_key.map(StatsRow),
            r#buff_visuals_key: x.r#buff_visuals_key.map(BuffVisualsRow),
            r#buff_category: x.r#buff_category.clone(),
            r#buff_limit: x.r#buff_limit.clone(),
            r#id2: x.r#id2.clone(),
            r#is_recovery: x.r#is_recovery.clone(),
            r#binary_stats: x.r#binary_stats.iter().copied().map(StatsRow).collect(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BuffDefinitionsRow(pub usize);

impl Deref for BuffDefinitionsRow {
    type Target = BuffDefinitions;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_BuffDefinitions[self.0]
    }
}

impl BuffDefinitionsRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BuffDefinitions {
        &TABLE_BuffDefinitions[self.0]
    }
    pub fn get(&self) -> &'static BuffDefinitions {
        &TABLE_BuffDefinitions[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BuffDefinitions
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BuffDefinitions)> {
        TABLE_BuffDefinitions
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_BuffDefinitions: LazyLock<Vec<BuffDefinitionsRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/BuffDefinitions.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct BuffDefinitionsRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "Description")]
    pub r#description: String,
    #[serde(rename = "Invisible")]
    pub r#invisible: bool,
    #[serde(rename = "Removable")]
    pub r#removable: bool,
    #[serde(rename = "Name")]
    pub r#name: String,
    #[serde(rename = "StatsKeys")]
    pub r#stats_keys: Vec<usize>,
    #[serde(rename = "Maximum_StatsKey")]
    pub r#maximum_stats_key: Option<usize>,
    #[serde(rename = "Current_StatsKey")]
    pub r#current_stats_key: Option<usize>,
    #[serde(rename = "BuffVisualsKey")]
    pub r#buff_visuals_key: Option<usize>,
    #[serde(rename = "BuffCategory")]
    pub r#buff_category: i32,
    #[serde(rename = "BuffLimit")]
    pub r#buff_limit: i32,
    #[serde(rename = "Id2")]
    pub r#id2: String,
    #[serde(rename = "IsRecovery")]
    pub r#is_recovery: bool,
    #[serde(rename = "BinaryStats")]
    pub r#binary_stats: Vec<usize>,
}
