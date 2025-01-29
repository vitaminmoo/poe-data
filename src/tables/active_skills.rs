#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct ActiveSkills {
    pub r#id: String,
    pub r#displayed_name: String,
    pub r#description: String,
    pub r#action_type: Option<ActionTypesRow>,
    pub r#icon_dds_file: String,
    pub r#active_skill_target_types: Vec<ActiveSkillTargetTypes>,
    pub r#active_skill_types: Vec<ActiveSkillTypeRow>,
    pub r#weapon_restriction_item_classes_keys: Vec<ItemClassesRow>,
    pub r#website_description: String,
    pub r#website_image: String,
    pub r#skill_totem_id: MaybeVariant<SkillTotems>,
    pub r#is_manually_casted: bool,
    pub r#input_stat_keys: Vec<StatsRow>,
    pub r#output_stat_keys: Vec<StatsRow>,
    pub r#minion_active_skill_types: Vec<ActiveSkillTypeRow>,
    pub r#alternate_skill_targeting_behaviours_key: Option<AlternateSkillTargetingBehavioursRow>,
    pub r#ai_file: String,
    pub r#transfigure_base: Option<ActiveSkillsRow>,
}

#[allow(non_upper_case_globals)]
pub static TABLE_ActiveSkills: LazyLock<Vec<ActiveSkills>> = LazyLock::new(|| {
    RAW_TABLE_ActiveSkills
        .iter()
        .map(|x| ActiveSkills {
            r#id: x.r#id.clone(),
            r#displayed_name: x.r#displayed_name.clone(),
            r#description: x.r#description.clone(),
            r#action_type: x.r#action_type.map(ActionTypesRow),
            r#icon_dds_file: x.r#icon_dds_file.clone(),
            r#active_skill_target_types: x
                .r#active_skill_target_types
                .iter()
                .map(|y| ActiveSkillTargetTypes::from_repr(*y).unwrap())
                .collect(),
            r#active_skill_types: x
                .r#active_skill_types
                .iter()
                .copied()
                .map(ActiveSkillTypeRow)
                .collect(),
            r#weapon_restriction_item_classes_keys: x
                .r#weapon_restriction_item_classes_keys
                .iter()
                .copied()
                .map(ItemClassesRow)
                .collect(),
            r#website_description: x.r#website_description.clone(),
            r#website_image: x.r#website_image.clone(),
            r#skill_totem_id: SkillTotems::from_repr(x.r#skill_totem_id).map_or(
                MaybeVariant::NotVariant(x.r#skill_totem_id),
                MaybeVariant::Variant,
            ),
            r#is_manually_casted: x.r#is_manually_casted.clone(),
            r#input_stat_keys: x.r#input_stat_keys.iter().copied().map(StatsRow).collect(),
            r#output_stat_keys: x.r#output_stat_keys.iter().copied().map(StatsRow).collect(),
            r#minion_active_skill_types: x
                .r#minion_active_skill_types
                .iter()
                .copied()
                .map(ActiveSkillTypeRow)
                .collect(),
            r#alternate_skill_targeting_behaviours_key: x
                .r#alternate_skill_targeting_behaviours_key
                .map(AlternateSkillTargetingBehavioursRow),
            r#ai_file: x.r#ai_file.clone(),
            r#transfigure_base: x.r#transfigure_base.map(ActiveSkillsRow),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ActiveSkillsRow(pub usize);

impl Deref for ActiveSkillsRow {
    type Target = ActiveSkills;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_ActiveSkills[self.0]
    }
}

impl ActiveSkillsRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ActiveSkills {
        &TABLE_ActiveSkills[self.0]
    }
    pub fn get(&self) -> &'static ActiveSkills {
        &TABLE_ActiveSkills[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ActiveSkills.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ActiveSkills)> {
        TABLE_ActiveSkills
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_ActiveSkills: LazyLock<Vec<ActiveSkillsRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/ActiveSkills.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct ActiveSkillsRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "DisplayedName")]
    pub r#displayed_name: String,
    #[serde(rename = "Description")]
    pub r#description: String,
    #[serde(rename = "ActionType")]
    pub r#action_type: Option<usize>,
    #[serde(rename = "Icon_DDSFile")]
    pub r#icon_dds_file: String,
    #[serde(rename = "ActiveSkillTargetTypes")]
    pub r#active_skill_target_types: Vec<usize>,
    #[serde(rename = "ActiveSkillTypes")]
    pub r#active_skill_types: Vec<usize>,
    #[serde(rename = "WeaponRestriction_ItemClassesKeys")]
    pub r#weapon_restriction_item_classes_keys: Vec<usize>,
    #[serde(rename = "WebsiteDescription")]
    pub r#website_description: String,
    #[serde(rename = "WebsiteImage")]
    pub r#website_image: String,
    #[serde(rename = "SkillTotemId")]
    pub r#skill_totem_id: usize,
    #[serde(rename = "IsManuallyCasted")]
    pub r#is_manually_casted: bool,
    #[serde(rename = "Input_StatKeys")]
    pub r#input_stat_keys: Vec<usize>,
    #[serde(rename = "Output_StatKeys")]
    pub r#output_stat_keys: Vec<usize>,
    #[serde(rename = "MinionActiveSkillTypes")]
    pub r#minion_active_skill_types: Vec<usize>,
    #[serde(rename = "AlternateSkillTargetingBehavioursKey")]
    pub r#alternate_skill_targeting_behaviours_key: Option<usize>,
    #[serde(rename = "AIFile")]
    pub r#ai_file: String,
    #[serde(rename = "TransfigureBase")]
    pub r#transfigure_base: Option<usize>,
}
