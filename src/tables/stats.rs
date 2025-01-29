#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct Stats {
    pub r#id: String,
    pub r#is_local: bool,
    pub r#is_weapon_local: bool,
    pub r#semantics: MaybeVariant<StatSemantics>,
    pub r#text: String,
    pub r#is_virtual: bool,
    pub r#main_hand_alias_stats_key: Option<StatsRow>,
    pub r#off_hand_alias_stats_key: Option<StatsRow>,
    pub r#hash32: i32,
    // column_ref_array belongs_active_skills_key
    pub r#category: Option<PassiveSkillStatCategoriesRow>,
    pub r#is_scalable: bool,
    pub r#context_flags: Vec<VirtualStatContextFlagsRow>,
}

#[allow(non_upper_case_globals)]
pub static TABLE_Stats: LazyLock<Vec<Stats>> = LazyLock::new(|| {
    RAW_TABLE_Stats
        .iter()
        .map(|x| {
            Stats {
                r#id: x.r#id.clone(),
                r#is_local: x.r#is_local.clone(),
                r#is_weapon_local: x.r#is_weapon_local.clone(),
                r#semantics: StatSemantics::from_repr(x.r#semantics).map_or(
                    MaybeVariant::NotVariant(x.r#semantics),
                    MaybeVariant::Variant,
                ),
                r#text: x.r#text.clone(),
                r#is_virtual: x.r#is_virtual.clone(),
                r#main_hand_alias_stats_key: x.r#main_hand_alias_stats_key.map(StatsRow),
                r#off_hand_alias_stats_key: x.r#off_hand_alias_stats_key.map(StatsRow),
                r#hash32: x.r#hash32.clone(),
                // column_ref_array belongs_active_skills_key
                r#category: x.r#category.map(PassiveSkillStatCategoriesRow),
                r#is_scalable: x.r#is_scalable.clone(),
                r#context_flags: x
                    .r#context_flags
                    .iter()
                    .copied()
                    .map(VirtualStatContextFlagsRow)
                    .collect(),
            }
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct StatsRow(pub usize);

impl Deref for StatsRow {
    type Target = Stats;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_Stats[self.0]
    }
}

impl StatsRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static Stats {
        &TABLE_Stats[self.0]
    }
    pub fn get(&self) -> &'static Stats {
        &TABLE_Stats[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Stats.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static Stats)> {
        TABLE_Stats.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_Stats: LazyLock<Vec<StatsRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/Stats.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct StatsRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "IsLocal")]
    pub r#is_local: bool,
    #[serde(rename = "IsWeaponLocal")]
    pub r#is_weapon_local: bool,
    #[serde(rename = "Semantics")]
    pub r#semantics: usize,
    #[serde(rename = "Text")]
    pub r#text: String,
    #[serde(rename = "IsVirtual")]
    pub r#is_virtual: bool,
    #[serde(rename = "MainHandAlias_StatsKey")]
    pub r#main_hand_alias_stats_key: Option<usize>,
    #[serde(rename = "OffHandAlias_StatsKey")]
    pub r#off_hand_alias_stats_key: Option<usize>,
    #[serde(rename = "HASH32")]
    pub r#hash32: i32,
    #[serde(rename = "BelongsActiveSkillsKey")]
    pub r#belongs_active_skills_key: Vec<String>,
    #[serde(rename = "Category")]
    pub r#category: Option<usize>,
    #[serde(rename = "IsScalable")]
    pub r#is_scalable: bool,
    #[serde(rename = "ContextFlags")]
    pub r#context_flags: Vec<usize>,
}
