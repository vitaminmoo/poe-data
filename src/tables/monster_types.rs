#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct MonsterTypes {
    pub r#id: String,
    pub r#is_summoned: bool,
    pub r#armour: i32,
    pub r#evasion: i32,
    pub r#energy_shield_from_life: i32,
    pub r#damage_spread: i32,
    pub r#monster_resistances_key: Option<MonsterResistancesRow>,
    pub r#is_large_abyss_monster: bool,
    pub r#is_small_abyss_monster: bool,
}

#[allow(non_upper_case_globals)]
pub static TABLE_MonsterTypes: LazyLock<Vec<MonsterTypes>> = LazyLock::new(|| {
    RAW_TABLE_MonsterTypes
        .iter()
        .map(|x| MonsterTypes {
            r#id: x.r#id.clone(),
            r#is_summoned: x.r#is_summoned.clone(),
            r#armour: x.r#armour.clone(),
            r#evasion: x.r#evasion.clone(),
            r#energy_shield_from_life: x.r#energy_shield_from_life.clone(),
            r#damage_spread: x.r#damage_spread.clone(),
            r#monster_resistances_key: x.r#monster_resistances_key.map(MonsterResistancesRow),
            r#is_large_abyss_monster: x.r#is_large_abyss_monster.clone(),
            r#is_small_abyss_monster: x.r#is_small_abyss_monster.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MonsterTypesRow(pub usize);

impl Deref for MonsterTypesRow {
    type Target = MonsterTypes;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_MonsterTypes[self.0]
    }
}

impl MonsterTypesRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MonsterTypes {
        &TABLE_MonsterTypes[self.0]
    }
    pub fn get(&self) -> &'static MonsterTypes {
        &TABLE_MonsterTypes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MonsterTypes.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MonsterTypes)> {
        TABLE_MonsterTypes
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_MonsterTypes: LazyLock<Vec<MonsterTypesRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/MonsterTypes.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct MonsterTypesRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "IsSummoned")]
    pub r#is_summoned: bool,
    #[serde(rename = "Armour")]
    pub r#armour: i32,
    #[serde(rename = "Evasion")]
    pub r#evasion: i32,
    #[serde(rename = "EnergyShieldFromLife")]
    pub r#energy_shield_from_life: i32,
    #[serde(rename = "DamageSpread")]
    pub r#damage_spread: i32,
    #[serde(rename = "MonsterResistancesKey")]
    pub r#monster_resistances_key: Option<usize>,
    #[serde(rename = "IsLargeAbyssMonster")]
    pub r#is_large_abyss_monster: bool,
    #[serde(rename = "IsSmallAbyssMonster")]
    pub r#is_small_abyss_monster: bool,
}
