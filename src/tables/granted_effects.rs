#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct GrantedEffects {
    pub r#id: String,
    pub r#is_support: bool,
    pub r#allowed_active_skill_types: Vec<ActiveSkillTypeRow>,
    pub r#support_gem_letter: String,
    pub r#attribute: MaybeVariant<Attributes>,
    pub r#added_active_skill_types: Vec<ActiveSkillTypeRow>,
    pub r#excluded_active_skill_types: Vec<ActiveSkillTypeRow>,
    pub r#supports_gems_only: bool,
    pub r#cannot_be_supported: bool,
    pub r#cast_time: i32,
    pub r#active_skill: Option<ActiveSkillsRow>,
    pub r#ignore_minion_types: bool,
    pub r#added_minion_active_skill_types: Vec<ActiveSkillTypeRow>,
    pub r#animation: Option<AnimationRow>,
    pub r#multi_part_achievement: Option<MultiPartAchievementsRow>,
    pub r#support_weapon_restrictions: Vec<ItemClassesRow>,
    pub r#regular_variant: Option<GrantedEffectsRow>,
    pub r#stat_set: Option<GrantedEffectStatSetsRow>,
}

#[allow(non_upper_case_globals)]
pub static TABLE_GrantedEffects: LazyLock<Vec<GrantedEffects>> = LazyLock::new(|| {
    RAW_TABLE_GrantedEffects
        .iter()
        .map(|x| GrantedEffects {
            r#id: x.r#id.clone(),
            r#is_support: x.r#is_support.clone(),
            r#allowed_active_skill_types: x
                .r#allowed_active_skill_types
                .iter()
                .copied()
                .map(ActiveSkillTypeRow)
                .collect(),
            r#support_gem_letter: x.r#support_gem_letter.clone(),
            r#attribute: Attributes::from_repr(x.r#attribute).map_or(
                MaybeVariant::NotVariant(x.r#attribute),
                MaybeVariant::Variant,
            ),
            r#added_active_skill_types: x
                .r#added_active_skill_types
                .iter()
                .copied()
                .map(ActiveSkillTypeRow)
                .collect(),
            r#excluded_active_skill_types: x
                .r#excluded_active_skill_types
                .iter()
                .copied()
                .map(ActiveSkillTypeRow)
                .collect(),
            r#supports_gems_only: x.r#supports_gems_only.clone(),
            r#cannot_be_supported: x.r#cannot_be_supported.clone(),
            r#cast_time: x.r#cast_time.clone(),
            r#active_skill: x.r#active_skill.map(ActiveSkillsRow),
            r#ignore_minion_types: x.r#ignore_minion_types.clone(),
            r#added_minion_active_skill_types: x
                .r#added_minion_active_skill_types
                .iter()
                .copied()
                .map(ActiveSkillTypeRow)
                .collect(),
            r#animation: x.r#animation.map(AnimationRow),
            r#multi_part_achievement: x.r#multi_part_achievement.map(MultiPartAchievementsRow),
            r#support_weapon_restrictions: x
                .r#support_weapon_restrictions
                .iter()
                .copied()
                .map(ItemClassesRow)
                .collect(),
            r#regular_variant: x.r#regular_variant.map(GrantedEffectsRow),
            r#stat_set: x.r#stat_set.map(GrantedEffectStatSetsRow),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct GrantedEffectsRow(pub usize);

impl Deref for GrantedEffectsRow {
    type Target = GrantedEffects;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_GrantedEffects[self.0]
    }
}

impl GrantedEffectsRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static GrantedEffects {
        &TABLE_GrantedEffects[self.0]
    }
    pub fn get(&self) -> &'static GrantedEffects {
        &TABLE_GrantedEffects[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_GrantedEffects
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static GrantedEffects)> {
        TABLE_GrantedEffects
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_GrantedEffects: LazyLock<Vec<GrantedEffectsRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/GrantedEffects.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct GrantedEffectsRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "IsSupport")]
    pub r#is_support: bool,
    #[serde(rename = "AllowedActiveSkillTypes")]
    pub r#allowed_active_skill_types: Vec<usize>,
    #[serde(rename = "SupportGemLetter")]
    pub r#support_gem_letter: String,
    #[serde(rename = "Attribute")]
    pub r#attribute: usize,
    #[serde(rename = "AddedActiveSkillTypes")]
    pub r#added_active_skill_types: Vec<usize>,
    #[serde(rename = "ExcludedActiveSkillTypes")]
    pub r#excluded_active_skill_types: Vec<usize>,
    #[serde(rename = "SupportsGemsOnly")]
    pub r#supports_gems_only: bool,
    #[serde(rename = "CannotBeSupported")]
    pub r#cannot_be_supported: bool,
    #[serde(rename = "CastTime")]
    pub r#cast_time: i32,
    #[serde(rename = "ActiveSkill")]
    pub r#active_skill: Option<usize>,
    #[serde(rename = "IgnoreMinionTypes")]
    pub r#ignore_minion_types: bool,
    #[serde(rename = "AddedMinionActiveSkillTypes")]
    pub r#added_minion_active_skill_types: Vec<usize>,
    #[serde(rename = "Animation")]
    pub r#animation: Option<usize>,
    #[serde(rename = "MultiPartAchievement")]
    pub r#multi_part_achievement: Option<usize>,
    #[serde(rename = "SupportWeaponRestrictions")]
    pub r#support_weapon_restrictions: Vec<usize>,
    #[serde(rename = "RegularVariant")]
    pub r#regular_variant: Option<usize>,
    #[serde(rename = "StatSet")]
    pub r#stat_set: Option<usize>,
}
