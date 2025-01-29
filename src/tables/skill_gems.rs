#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct SkillGems {
    pub r#base_item_types_key: Option<BaseItemTypesRow>,
    pub r#strength_requirement_percent: i32,
    pub r#dexterity_requirement_percent: i32,
    pub r#intelligence_requirement_percent: i32,
    pub r#vaal_variant_base_item_types_key: Option<BaseItemTypesRow>,
    pub r#is_vaal_variant: bool,
    pub r#minion_global_skill_level_stat: Option<StatsRow>,
    pub r#is_support: bool,
    pub r#awakened_variant: Option<SkillGemsRow>,
    pub r#regular_variant: Option<SkillGemsRow>,
    pub r#item_experience_type: Option<ItemExperienceTypesRow>,
    pub r#mtx_slot_types: Vec<MicrotransactionSkillGemEffectSlotTypesRow>,
    pub r#gem_effects: Vec<GemEffectsRow>,
}

#[allow(non_upper_case_globals)]
pub static TABLE_SkillGems: LazyLock<Vec<SkillGems>> = LazyLock::new(|| {
    RAW_TABLE_SkillGems
        .iter()
        .map(|x| SkillGems {
            r#base_item_types_key: x.r#base_item_types_key.map(BaseItemTypesRow),
            r#strength_requirement_percent: x.r#strength_requirement_percent.clone(),
            r#dexterity_requirement_percent: x.r#dexterity_requirement_percent.clone(),
            r#intelligence_requirement_percent: x.r#intelligence_requirement_percent.clone(),
            r#vaal_variant_base_item_types_key: x
                .r#vaal_variant_base_item_types_key
                .map(BaseItemTypesRow),
            r#is_vaal_variant: x.r#is_vaal_variant.clone(),
            r#minion_global_skill_level_stat: x.r#minion_global_skill_level_stat.map(StatsRow),
            r#is_support: x.r#is_support.clone(),
            r#awakened_variant: x.r#awakened_variant.map(SkillGemsRow),
            r#regular_variant: x.r#regular_variant.map(SkillGemsRow),
            r#item_experience_type: x.r#item_experience_type.map(ItemExperienceTypesRow),
            r#mtx_slot_types: x
                .r#mtx_slot_types
                .iter()
                .copied()
                .map(MicrotransactionSkillGemEffectSlotTypesRow)
                .collect(),
            r#gem_effects: x.r#gem_effects.iter().copied().map(GemEffectsRow).collect(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct SkillGemsRow(pub usize);

impl Deref for SkillGemsRow {
    type Target = SkillGems;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_SkillGems[self.0]
    }
}

impl SkillGemsRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static SkillGems {
        &TABLE_SkillGems[self.0]
    }
    pub fn get(&self) -> &'static SkillGems {
        &TABLE_SkillGems[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_SkillGems.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static SkillGems)> {
        TABLE_SkillGems
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_SkillGems: LazyLock<Vec<SkillGemsRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/SkillGems.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct SkillGemsRaw {
    #[serde(rename = "BaseItemTypesKey")]
    pub r#base_item_types_key: Option<usize>,
    #[serde(rename = "StrengthRequirementPercent")]
    pub r#strength_requirement_percent: i32,
    #[serde(rename = "DexterityRequirementPercent")]
    pub r#dexterity_requirement_percent: i32,
    #[serde(rename = "IntelligenceRequirementPercent")]
    pub r#intelligence_requirement_percent: i32,
    #[serde(rename = "VaalVariant_BaseItemTypesKey")]
    pub r#vaal_variant_base_item_types_key: Option<usize>,
    #[serde(rename = "IsVaalVariant")]
    pub r#is_vaal_variant: bool,
    #[serde(rename = "MinionGlobalSkillLevelStat")]
    pub r#minion_global_skill_level_stat: Option<usize>,
    #[serde(rename = "IsSupport")]
    pub r#is_support: bool,
    #[serde(rename = "AwakenedVariant")]
    pub r#awakened_variant: Option<usize>,
    #[serde(rename = "RegularVariant")]
    pub r#regular_variant: Option<usize>,
    #[serde(rename = "ItemExperienceType")]
    pub r#item_experience_type: Option<usize>,
    #[serde(rename = "MtxSlotTypes")]
    pub r#mtx_slot_types: Vec<usize>,
    #[serde(rename = "GemEffects")]
    pub r#gem_effects: Vec<usize>,
}
