#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct BuffTemplates {
    pub r#id: String,
    pub r#buff_definitions_key: Option<BuffDefinitionsRow>,
    pub r#buff_stat_values: Vec<i32>,
    pub r#aura_radius: i32,
    pub r#buff_visuals_key: Option<BuffVisualsRow>,
    pub r#stats_key: Vec<StatsRow>,
}

#[allow(non_upper_case_globals)]
pub static TABLE_BuffTemplates: LazyLock<Vec<BuffTemplates>> = LazyLock::new(|| {
    RAW_TABLE_BuffTemplates
        .iter()
        .map(|x| BuffTemplates {
            r#id: x.r#id.clone(),
            r#buff_definitions_key: x.r#buff_definitions_key.map(BuffDefinitionsRow),
            r#buff_stat_values: x.r#buff_stat_values.clone(),
            r#aura_radius: x.r#aura_radius.clone(),
            r#buff_visuals_key: x.r#buff_visuals_key.map(BuffVisualsRow),
            r#stats_key: x.r#stats_key.iter().copied().map(StatsRow).collect(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BuffTemplatesRow(pub usize);

impl Deref for BuffTemplatesRow {
    type Target = BuffTemplates;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_BuffTemplates[self.0]
    }
}

impl BuffTemplatesRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BuffTemplates {
        &TABLE_BuffTemplates[self.0]
    }
    pub fn get(&self) -> &'static BuffTemplates {
        &TABLE_BuffTemplates[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BuffTemplates.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BuffTemplates)> {
        TABLE_BuffTemplates
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_BuffTemplates: LazyLock<Vec<BuffTemplatesRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/BuffTemplates.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct BuffTemplatesRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "BuffDefinitionsKey")]
    pub r#buff_definitions_key: Option<usize>,
    #[serde(rename = "Buff_StatValues")]
    pub r#buff_stat_values: Vec<i32>,
    #[serde(rename = "AuraRadius")]
    pub r#aura_radius: i32,
    #[serde(rename = "BuffVisualsKey")]
    pub r#buff_visuals_key: Option<usize>,
    #[serde(rename = "StatsKey")]
    pub r#stats_key: Vec<usize>,
}
