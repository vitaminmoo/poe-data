#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct GrantedEffectStatSets {
    pub r#id: String,
    pub r#implicit_stats: Vec<StatsRow>,
    pub r#constant_stats: Vec<StatsRow>,
    pub r#constant_stats_values: Vec<i32>,
    pub r#base_effectiveness: f32,
    pub r#incremental_effectiveness: f32,
}

#[allow(non_upper_case_globals)]
pub static TABLE_GrantedEffectStatSets: LazyLock<Vec<GrantedEffectStatSets>> =
    LazyLock::new(|| {
        RAW_TABLE_GrantedEffectStatSets
            .iter()
            .map(|x| GrantedEffectStatSets {
                r#id: x.r#id.clone(),
                r#implicit_stats: x.r#implicit_stats.iter().copied().map(StatsRow).collect(),
                r#constant_stats: x.r#constant_stats.iter().copied().map(StatsRow).collect(),
                r#constant_stats_values: x.r#constant_stats_values.clone(),
                r#base_effectiveness: x.r#base_effectiveness.clone(),
                r#incremental_effectiveness: x.r#incremental_effectiveness.clone(),
            })
            .collect()
    });

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct GrantedEffectStatSetsRow(pub usize);

impl Deref for GrantedEffectStatSetsRow {
    type Target = GrantedEffectStatSets;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_GrantedEffectStatSets[self.0]
    }
}

impl GrantedEffectStatSetsRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static GrantedEffectStatSets {
        &TABLE_GrantedEffectStatSets[self.0]
    }
    pub fn get(&self) -> &'static GrantedEffectStatSets {
        &TABLE_GrantedEffectStatSets[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_GrantedEffectStatSets
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static GrantedEffectStatSets)> {
        TABLE_GrantedEffectStatSets
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_GrantedEffectStatSets: LazyLock<Vec<GrantedEffectStatSetsRaw>> =
    LazyLock::new(|| {
        const DATA: &str = include_str!("../../data/tables/English/GrantedEffectStatSets.json");
        serde_json::from_str(DATA).unwrap()
    });

#[derive(Debug, Deserialize, Serialize)]
struct GrantedEffectStatSetsRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "ImplicitStats")]
    pub r#implicit_stats: Vec<usize>,
    #[serde(rename = "ConstantStats")]
    pub r#constant_stats: Vec<usize>,
    #[serde(rename = "ConstantStatsValues")]
    pub r#constant_stats_values: Vec<i32>,
    #[serde(rename = "BaseEffectiveness")]
    pub r#base_effectiveness: f32,
    #[serde(rename = "IncrementalEffectiveness")]
    pub r#incremental_effectiveness: f32,
}
