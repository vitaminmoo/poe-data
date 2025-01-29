#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct MonsterResistances {
    pub r#id: String,
    pub r#fire_normal: i32,
    pub r#cold_normal: i32,
    pub r#lightning_normal: i32,
    pub r#chaos_normal: i32,
    pub r#fire_cruel: i32,
    pub r#cold_cruel: i32,
    pub r#lightning_cruel: i32,
    pub r#chaos_cruel: i32,
    pub r#fire_merciless: i32,
    pub r#cold_merciless: i32,
    pub r#lightning_merciless: i32,
    pub r#chaos_merciless: i32,
}

#[allow(non_upper_case_globals)]
pub static TABLE_MonsterResistances: LazyLock<Vec<MonsterResistances>> = LazyLock::new(|| {
    RAW_TABLE_MonsterResistances
        .iter()
        .map(|x| MonsterResistances {
            r#id: x.r#id.clone(),
            r#fire_normal: x.r#fire_normal.clone(),
            r#cold_normal: x.r#cold_normal.clone(),
            r#lightning_normal: x.r#lightning_normal.clone(),
            r#chaos_normal: x.r#chaos_normal.clone(),
            r#fire_cruel: x.r#fire_cruel.clone(),
            r#cold_cruel: x.r#cold_cruel.clone(),
            r#lightning_cruel: x.r#lightning_cruel.clone(),
            r#chaos_cruel: x.r#chaos_cruel.clone(),
            r#fire_merciless: x.r#fire_merciless.clone(),
            r#cold_merciless: x.r#cold_merciless.clone(),
            r#lightning_merciless: x.r#lightning_merciless.clone(),
            r#chaos_merciless: x.r#chaos_merciless.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MonsterResistancesRow(pub usize);

impl Deref for MonsterResistancesRow {
    type Target = MonsterResistances;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_MonsterResistances[self.0]
    }
}

impl MonsterResistancesRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MonsterResistances {
        &TABLE_MonsterResistances[self.0]
    }
    pub fn get(&self) -> &'static MonsterResistances {
        &TABLE_MonsterResistances[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MonsterResistances
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MonsterResistances)> {
        TABLE_MonsterResistances
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_MonsterResistances: LazyLock<Vec<MonsterResistancesRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/MonsterResistances.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct MonsterResistancesRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "FireNormal")]
    pub r#fire_normal: i32,
    #[serde(rename = "ColdNormal")]
    pub r#cold_normal: i32,
    #[serde(rename = "LightningNormal")]
    pub r#lightning_normal: i32,
    #[serde(rename = "ChaosNormal")]
    pub r#chaos_normal: i32,
    #[serde(rename = "FireCruel")]
    pub r#fire_cruel: i32,
    #[serde(rename = "ColdCruel")]
    pub r#cold_cruel: i32,
    #[serde(rename = "LightningCruel")]
    pub r#lightning_cruel: i32,
    #[serde(rename = "ChaosCruel")]
    pub r#chaos_cruel: i32,
    #[serde(rename = "FireMerciless")]
    pub r#fire_merciless: i32,
    #[serde(rename = "ColdMerciless")]
    pub r#cold_merciless: i32,
    #[serde(rename = "LightningMerciless")]
    pub r#lightning_merciless: i32,
    #[serde(rename = "ChaosMerciless")]
    pub r#chaos_merciless: i32,
}
