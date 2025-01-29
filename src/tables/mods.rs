#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct Mods {
    pub r#id: String,
    pub r#hash16: i16,
    pub r#mod_type_key: Option<ModTypeRow>,
    pub r#level: i32,
    pub r#stats_key1: Option<StatsRow>,
    pub r#stats_key2: Option<StatsRow>,
    pub r#stats_key3: Option<StatsRow>,
    pub r#stats_key4: Option<StatsRow>,
    pub r#domain: MaybeVariant<ModDomains>,
    pub r#name: String,
    pub r#generation_type: MaybeVariant<ModGenerationType>,
    pub r#families: Vec<ModFamilyRow>,
    pub r#stat1_min: i32,
    pub r#stat1_max: i32,
    pub r#stat2_min: i32,
    pub r#stat2_max: i32,
    pub r#stat3_min: i32,
    pub r#stat3_max: i32,
    pub r#stat4_min: i32,
    pub r#stat4_max: i32,
    pub r#spawn_weight_tags_keys: Vec<TagsRow>,
    pub r#spawn_weight_values: Vec<i32>,
    pub r#tags_keys: Vec<TagsRow>,
    pub r#granted_effects_per_level_keys: Vec<GrantedEffectsPerLevelRow>,
    pub r#monster_metadata: String,
    pub r#monster_kill_achievements: Vec<AchievementItemsRow>,
    pub r#chest_mod_type: Vec<ModTypeRow>,
    pub r#stat5_min: i32,
    pub r#stat5_max: i32,
    pub r#stats_key5: Option<StatsRow>,
    pub r#full_area_clear_achievement_items_key: Vec<AchievementItemsRow>,
    pub r#achievement_items_key: Vec<AchievementItemsRow>,
    pub r#generation_weight_tags_keys: Vec<TagsRow>,
    pub r#generation_weight_values: Vec<i32>,
    pub r#modify_maps_achievements: Vec<AchievementItemsRow>,
    pub r#is_essence_only_modifier: bool,
    pub r#stat6_min: i32,
    pub r#stat6_max: i32,
    pub r#stats_key6: Option<StatsRow>,
    pub r#max_level: i32,
    pub r#crafting_item_class_restrictions: Vec<ItemClassesRow>,
    pub r#monster_on_death: String,
    pub r#heist_sub_stat_value1: i32,
    pub r#heist_sub_stat_value2: i32,
    pub r#heist_stats_key0: Option<StatsRow>,
    pub r#heist_stats_key1: Option<StatsRow>,
    pub r#heist_add_stat_value1: i32,
    pub r#heist_add_stat_value2: i32,
    pub r#influence_types: MaybeVariant<InfluenceTypes>,
    pub r#implicit_tags_keys: Vec<TagsRow>,
    pub r#buff_template: Option<BuffTemplatesRow>,
    pub r#archnemesis_minion_mod: Option<ModsRow>,
    pub r#hash32: i32,
}

#[allow(non_upper_case_globals)]
pub static TABLE_Mods: LazyLock<Vec<Mods>> = LazyLock::new(|| {
    RAW_TABLE_Mods
        .iter()
        .map(|x| Mods {
            r#id: x.r#id.clone(),
            r#hash16: x.r#hash16.clone(),
            r#mod_type_key: x.r#mod_type_key.map(ModTypeRow),
            r#level: x.r#level.clone(),
            r#stats_key1: x.r#stats_key1.map(StatsRow),
            r#stats_key2: x.r#stats_key2.map(StatsRow),
            r#stats_key3: x.r#stats_key3.map(StatsRow),
            r#stats_key4: x.r#stats_key4.map(StatsRow),
            r#domain: ModDomains::from_repr(x.r#domain)
                .map_or(MaybeVariant::NotVariant(x.r#domain), MaybeVariant::Variant),
            r#name: x.r#name.clone(),
            r#generation_type: ModGenerationType::from_repr(x.r#generation_type).map_or(
                MaybeVariant::NotVariant(x.r#generation_type),
                MaybeVariant::Variant,
            ),
            r#families: x.r#families.iter().copied().map(ModFamilyRow).collect(),
            r#stat1_min: x.r#stat1_min.clone(),
            r#stat1_max: x.r#stat1_max.clone(),
            r#stat2_min: x.r#stat2_min.clone(),
            r#stat2_max: x.r#stat2_max.clone(),
            r#stat3_min: x.r#stat3_min.clone(),
            r#stat3_max: x.r#stat3_max.clone(),
            r#stat4_min: x.r#stat4_min.clone(),
            r#stat4_max: x.r#stat4_max.clone(),
            r#spawn_weight_tags_keys: x
                .r#spawn_weight_tags_keys
                .iter()
                .copied()
                .map(TagsRow)
                .collect(),
            r#spawn_weight_values: x.r#spawn_weight_values.clone(),
            r#tags_keys: x.r#tags_keys.iter().copied().map(TagsRow).collect(),
            r#granted_effects_per_level_keys: x
                .r#granted_effects_per_level_keys
                .iter()
                .copied()
                .map(GrantedEffectsPerLevelRow)
                .collect(),
            r#monster_metadata: x.r#monster_metadata.clone(),
            r#monster_kill_achievements: x
                .r#monster_kill_achievements
                .iter()
                .copied()
                .map(AchievementItemsRow)
                .collect(),
            r#chest_mod_type: x.r#chest_mod_type.iter().copied().map(ModTypeRow).collect(),
            r#stat5_min: x.r#stat5_min.clone(),
            r#stat5_max: x.r#stat5_max.clone(),
            r#stats_key5: x.r#stats_key5.map(StatsRow),
            r#full_area_clear_achievement_items_key: x
                .r#full_area_clear_achievement_items_key
                .iter()
                .copied()
                .map(AchievementItemsRow)
                .collect(),
            r#achievement_items_key: x
                .r#achievement_items_key
                .iter()
                .copied()
                .map(AchievementItemsRow)
                .collect(),
            r#generation_weight_tags_keys: x
                .r#generation_weight_tags_keys
                .iter()
                .copied()
                .map(TagsRow)
                .collect(),
            r#generation_weight_values: x.r#generation_weight_values.clone(),
            r#modify_maps_achievements: x
                .r#modify_maps_achievements
                .iter()
                .copied()
                .map(AchievementItemsRow)
                .collect(),
            r#is_essence_only_modifier: x.r#is_essence_only_modifier.clone(),
            r#stat6_min: x.r#stat6_min.clone(),
            r#stat6_max: x.r#stat6_max.clone(),
            r#stats_key6: x.r#stats_key6.map(StatsRow),
            r#max_level: x.r#max_level.clone(),
            r#crafting_item_class_restrictions: x
                .r#crafting_item_class_restrictions
                .iter()
                .copied()
                .map(ItemClassesRow)
                .collect(),
            r#monster_on_death: x.r#monster_on_death.clone(),
            r#heist_sub_stat_value1: x.r#heist_sub_stat_value1.clone(),
            r#heist_sub_stat_value2: x.r#heist_sub_stat_value2.clone(),
            r#heist_stats_key0: x.r#heist_stats_key0.map(StatsRow),
            r#heist_stats_key1: x.r#heist_stats_key1.map(StatsRow),
            r#heist_add_stat_value1: x.r#heist_add_stat_value1.clone(),
            r#heist_add_stat_value2: x.r#heist_add_stat_value2.clone(),
            r#influence_types: InfluenceTypes::from_repr(x.r#influence_types).map_or(
                MaybeVariant::NotVariant(x.r#influence_types),
                MaybeVariant::Variant,
            ),
            r#implicit_tags_keys: x
                .r#implicit_tags_keys
                .iter()
                .copied()
                .map(TagsRow)
                .collect(),
            r#buff_template: x.r#buff_template.map(BuffTemplatesRow),
            r#archnemesis_minion_mod: x.r#archnemesis_minion_mod.map(ModsRow),
            r#hash32: x.r#hash32.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ModsRow(pub usize);

impl Deref for ModsRow {
    type Target = Mods;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_Mods[self.0]
    }
}

impl ModsRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static Mods {
        &TABLE_Mods[self.0]
    }
    pub fn get(&self) -> &'static Mods {
        &TABLE_Mods[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Mods.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static Mods)> {
        TABLE_Mods.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_Mods: LazyLock<Vec<ModsRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/Mods.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct ModsRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "HASH16")]
    pub r#hash16: i16,
    #[serde(rename = "ModTypeKey")]
    pub r#mod_type_key: Option<usize>,
    #[serde(rename = "Level")]
    pub r#level: i32,
    #[serde(rename = "StatsKey1")]
    pub r#stats_key1: Option<usize>,
    #[serde(rename = "StatsKey2")]
    pub r#stats_key2: Option<usize>,
    #[serde(rename = "StatsKey3")]
    pub r#stats_key3: Option<usize>,
    #[serde(rename = "StatsKey4")]
    pub r#stats_key4: Option<usize>,
    #[serde(rename = "Domain")]
    pub r#domain: usize,
    #[serde(rename = "Name")]
    pub r#name: String,
    #[serde(rename = "GenerationType")]
    pub r#generation_type: usize,
    #[serde(rename = "Families")]
    pub r#families: Vec<usize>,
    #[serde(rename = "Stat1Min")]
    pub r#stat1_min: i32,
    #[serde(rename = "Stat1Max")]
    pub r#stat1_max: i32,
    #[serde(rename = "Stat2Min")]
    pub r#stat2_min: i32,
    #[serde(rename = "Stat2Max")]
    pub r#stat2_max: i32,
    #[serde(rename = "Stat3Min")]
    pub r#stat3_min: i32,
    #[serde(rename = "Stat3Max")]
    pub r#stat3_max: i32,
    #[serde(rename = "Stat4Min")]
    pub r#stat4_min: i32,
    #[serde(rename = "Stat4Max")]
    pub r#stat4_max: i32,
    #[serde(rename = "SpawnWeight_TagsKeys")]
    pub r#spawn_weight_tags_keys: Vec<usize>,
    #[serde(rename = "SpawnWeight_Values")]
    pub r#spawn_weight_values: Vec<i32>,
    #[serde(rename = "TagsKeys")]
    pub r#tags_keys: Vec<usize>,
    #[serde(rename = "GrantedEffectsPerLevelKeys")]
    pub r#granted_effects_per_level_keys: Vec<usize>,
    #[serde(rename = "MonsterMetadata")]
    pub r#monster_metadata: String,
    #[serde(rename = "MonsterKillAchievements")]
    pub r#monster_kill_achievements: Vec<usize>,
    #[serde(rename = "ChestModType")]
    pub r#chest_mod_type: Vec<usize>,
    #[serde(rename = "Stat5Min")]
    pub r#stat5_min: i32,
    #[serde(rename = "Stat5Max")]
    pub r#stat5_max: i32,
    #[serde(rename = "StatsKey5")]
    pub r#stats_key5: Option<usize>,
    #[serde(rename = "FullAreaClear_AchievementItemsKey")]
    pub r#full_area_clear_achievement_items_key: Vec<usize>,
    #[serde(rename = "AchievementItemsKey")]
    pub r#achievement_items_key: Vec<usize>,
    #[serde(rename = "GenerationWeight_TagsKeys")]
    pub r#generation_weight_tags_keys: Vec<usize>,
    #[serde(rename = "GenerationWeight_Values")]
    pub r#generation_weight_values: Vec<i32>,
    #[serde(rename = "ModifyMapsAchievements")]
    pub r#modify_maps_achievements: Vec<usize>,
    #[serde(rename = "IsEssenceOnlyModifier")]
    pub r#is_essence_only_modifier: bool,
    #[serde(rename = "Stat6Min")]
    pub r#stat6_min: i32,
    #[serde(rename = "Stat6Max")]
    pub r#stat6_max: i32,
    #[serde(rename = "StatsKey6")]
    pub r#stats_key6: Option<usize>,
    #[serde(rename = "MaxLevel")]
    pub r#max_level: i32,
    #[serde(rename = "CraftingItemClassRestrictions")]
    pub r#crafting_item_class_restrictions: Vec<usize>,
    #[serde(rename = "MonsterOnDeath")]
    pub r#monster_on_death: String,
    #[serde(rename = "Heist_SubStatValue1")]
    pub r#heist_sub_stat_value1: i32,
    #[serde(rename = "Heist_SubStatValue2")]
    pub r#heist_sub_stat_value2: i32,
    #[serde(rename = "Heist_StatsKey0")]
    pub r#heist_stats_key0: Option<usize>,
    #[serde(rename = "Heist_StatsKey1")]
    pub r#heist_stats_key1: Option<usize>,
    #[serde(rename = "Heist_AddStatValue1")]
    pub r#heist_add_stat_value1: i32,
    #[serde(rename = "Heist_AddStatValue2")]
    pub r#heist_add_stat_value2: i32,
    #[serde(rename = "InfluenceTypes")]
    pub r#influence_types: usize,
    #[serde(rename = "ImplicitTagsKeys")]
    pub r#implicit_tags_keys: Vec<usize>,
    #[serde(rename = "BuffTemplate")]
    pub r#buff_template: Option<usize>,
    #[serde(rename = "ArchnemesisMinionMod")]
    pub r#archnemesis_minion_mod: Option<usize>,
    #[serde(rename = "HASH32")]
    pub r#hash32: i32,
}
