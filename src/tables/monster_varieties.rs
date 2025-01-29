#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct MonsterVarieties {
    pub r#id: String,
    pub r#monster_types_key: Option<MonsterTypesRow>,
    pub r#object_size: i32,
    pub r#minimum_attack_distance: i32,
    pub r#maximum_attack_distance: i32,
    pub r#act_files: Vec<String>,
    pub r#ao_files: Vec<String>,
    pub r#base_monster_type_index: String,
    pub r#mods_keys: Vec<ModsRow>,
    pub r#model_size_multiplier: i32,
    pub r#tags_keys: Vec<TagsRow>,
    pub r#experience_multiplier: i32,
    pub r#critical_strike_chance: i32,
    pub r#granted_effects_keys: Vec<GrantedEffectsRow>,
    pub r#ais_file: String,
    pub r#mods_keys2: Vec<ModsRow>,
    pub r#stance: String,
    pub r#name: String,
    pub r#damage_multiplier: i32,
    pub r#life_multiplier: i32,
    pub r#attack_speed: i32,
    pub r#weapon1_item_visual_identity_keys: Vec<ItemVisualIdentityRow>,
    pub r#weapon2_item_visual_identity_keys: Vec<ItemVisualIdentityRow>,
    pub r#back_item_visual_identity_key: Option<ItemVisualIdentityRow>,
    pub r#main_hand_item_classes_key: Option<ItemClassesRow>,
    pub r#off_hand_item_classes_key: Option<ItemClassesRow>,
    pub r#helmet_item_visual_identity_key: Option<ItemVisualIdentityRow>,
    pub r#kill_specific_monster_count_achievement_items_keys: Vec<AchievementItemsRow>,
    pub r#special_mods_keys: Vec<ModsRow>,
    pub r#kill_rare_achievement_items_keys: Vec<AchievementItemsRow>,
    pub r#hash16: i16,
    pub r#kill_while_onslaught_is_active_achievement_items_key: Option<AchievementItemsRow>,
    pub r#monster_segments_key: Option<MonsterSegmentsRow>,
    pub r#monster_armours_key: Option<MonsterArmoursRow>,
    pub r#kill_while_talisman_is_active_achievement_items_key: Option<AchievementItemsRow>,
    pub r#part1_mods_keys: Vec<ModsRow>,
    pub r#part2_mods_keys: Vec<ModsRow>,
    pub r#endgame_mods_keys: Vec<ModsRow>,
    pub r#sink_animation_ao_file: String,
    pub r#epk_file: String,
    pub r#monster_conditional_effect_packs_key: Option<MonsterConditionalEffectPacksRow>,
    pub r#addon_monster_type_index: Vec<String>,
    pub r#boss_health_bar: bool,
}

#[allow(non_upper_case_globals)]
pub static TABLE_MonsterVarieties: LazyLock<Vec<MonsterVarieties>> = LazyLock::new(|| {
    RAW_TABLE_MonsterVarieties
        .iter()
        .map(|x| MonsterVarieties {
            r#id: x.r#id.clone(),
            r#monster_types_key: x.r#monster_types_key.map(MonsterTypesRow),
            r#object_size: x.r#object_size.clone(),
            r#minimum_attack_distance: x.r#minimum_attack_distance.clone(),
            r#maximum_attack_distance: x.r#maximum_attack_distance.clone(),
            r#act_files: x.r#act_files.clone(),
            r#ao_files: x.r#ao_files.clone(),
            r#base_monster_type_index: x.r#base_monster_type_index.clone(),
            r#mods_keys: x.r#mods_keys.iter().copied().map(ModsRow).collect(),
            r#model_size_multiplier: x.r#model_size_multiplier.clone(),
            r#tags_keys: x.r#tags_keys.iter().copied().map(TagsRow).collect(),
            r#experience_multiplier: x.r#experience_multiplier.clone(),
            r#critical_strike_chance: x.r#critical_strike_chance.clone(),
            r#granted_effects_keys: x
                .r#granted_effects_keys
                .iter()
                .copied()
                .map(GrantedEffectsRow)
                .collect(),
            r#ais_file: x.r#ais_file.clone(),
            r#mods_keys2: x.r#mods_keys2.iter().copied().map(ModsRow).collect(),
            r#stance: x.r#stance.clone(),
            r#name: x.r#name.clone(),
            r#damage_multiplier: x.r#damage_multiplier.clone(),
            r#life_multiplier: x.r#life_multiplier.clone(),
            r#attack_speed: x.r#attack_speed.clone(),
            r#weapon1_item_visual_identity_keys: x
                .r#weapon1_item_visual_identity_keys
                .iter()
                .copied()
                .map(ItemVisualIdentityRow)
                .collect(),
            r#weapon2_item_visual_identity_keys: x
                .r#weapon2_item_visual_identity_keys
                .iter()
                .copied()
                .map(ItemVisualIdentityRow)
                .collect(),
            r#back_item_visual_identity_key: x
                .r#back_item_visual_identity_key
                .map(ItemVisualIdentityRow),
            r#main_hand_item_classes_key: x.r#main_hand_item_classes_key.map(ItemClassesRow),
            r#off_hand_item_classes_key: x.r#off_hand_item_classes_key.map(ItemClassesRow),
            r#helmet_item_visual_identity_key: x
                .r#helmet_item_visual_identity_key
                .map(ItemVisualIdentityRow),
            r#kill_specific_monster_count_achievement_items_keys: x
                .r#kill_specific_monster_count_achievement_items_keys
                .iter()
                .copied()
                .map(AchievementItemsRow)
                .collect(),
            r#special_mods_keys: x.r#special_mods_keys.iter().copied().map(ModsRow).collect(),
            r#kill_rare_achievement_items_keys: x
                .r#kill_rare_achievement_items_keys
                .iter()
                .copied()
                .map(AchievementItemsRow)
                .collect(),
            r#hash16: x.r#hash16.clone(),
            r#kill_while_onslaught_is_active_achievement_items_key: x
                .r#kill_while_onslaught_is_active_achievement_items_key
                .map(AchievementItemsRow),
            r#monster_segments_key: x.r#monster_segments_key.map(MonsterSegmentsRow),
            r#monster_armours_key: x.r#monster_armours_key.map(MonsterArmoursRow),
            r#kill_while_talisman_is_active_achievement_items_key: x
                .r#kill_while_talisman_is_active_achievement_items_key
                .map(AchievementItemsRow),
            r#part1_mods_keys: x.r#part1_mods_keys.iter().copied().map(ModsRow).collect(),
            r#part2_mods_keys: x.r#part2_mods_keys.iter().copied().map(ModsRow).collect(),
            r#endgame_mods_keys: x.r#endgame_mods_keys.iter().copied().map(ModsRow).collect(),
            r#sink_animation_ao_file: x.r#sink_animation_ao_file.clone(),
            r#epk_file: x.r#epk_file.clone(),
            r#monster_conditional_effect_packs_key: x
                .r#monster_conditional_effect_packs_key
                .map(MonsterConditionalEffectPacksRow),
            r#addon_monster_type_index: x.r#addon_monster_type_index.clone(),
            r#boss_health_bar: x.r#boss_health_bar.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MonsterVarietiesRow(pub usize);

impl Deref for MonsterVarietiesRow {
    type Target = MonsterVarieties;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_MonsterVarieties[self.0]
    }
}

impl MonsterVarietiesRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MonsterVarieties {
        &TABLE_MonsterVarieties[self.0]
    }
    pub fn get(&self) -> &'static MonsterVarieties {
        &TABLE_MonsterVarieties[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MonsterVarieties
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MonsterVarieties)> {
        TABLE_MonsterVarieties
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_MonsterVarieties: LazyLock<Vec<MonsterVarietiesRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/MonsterVarieties.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct MonsterVarietiesRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "MonsterTypesKey")]
    pub r#monster_types_key: Option<usize>,
    #[serde(rename = "ObjectSize")]
    pub r#object_size: i32,
    #[serde(rename = "MinimumAttackDistance")]
    pub r#minimum_attack_distance: i32,
    #[serde(rename = "MaximumAttackDistance")]
    pub r#maximum_attack_distance: i32,
    #[serde(rename = "ACTFiles")]
    pub r#act_files: Vec<String>,
    #[serde(rename = "AOFiles")]
    pub r#ao_files: Vec<String>,
    #[serde(rename = "BaseMonsterTypeIndex")]
    pub r#base_monster_type_index: String,
    #[serde(rename = "ModsKeys")]
    pub r#mods_keys: Vec<usize>,
    #[serde(rename = "ModelSizeMultiplier")]
    pub r#model_size_multiplier: i32,
    #[serde(rename = "TagsKeys")]
    pub r#tags_keys: Vec<usize>,
    #[serde(rename = "ExperienceMultiplier")]
    pub r#experience_multiplier: i32,
    #[serde(rename = "CriticalStrikeChance")]
    pub r#critical_strike_chance: i32,
    #[serde(rename = "GrantedEffectsKeys")]
    pub r#granted_effects_keys: Vec<usize>,
    #[serde(rename = "AISFile")]
    pub r#ais_file: String,
    #[serde(rename = "ModsKeys2")]
    pub r#mods_keys2: Vec<usize>,
    #[serde(rename = "Stance")]
    pub r#stance: String,
    #[serde(rename = "Name")]
    pub r#name: String,
    #[serde(rename = "DamageMultiplier")]
    pub r#damage_multiplier: i32,
    #[serde(rename = "LifeMultiplier")]
    pub r#life_multiplier: i32,
    #[serde(rename = "AttackSpeed")]
    pub r#attack_speed: i32,
    #[serde(rename = "Weapon1_ItemVisualIdentityKeys")]
    pub r#weapon1_item_visual_identity_keys: Vec<usize>,
    #[serde(rename = "Weapon2_ItemVisualIdentityKeys")]
    pub r#weapon2_item_visual_identity_keys: Vec<usize>,
    #[serde(rename = "Back_ItemVisualIdentityKey")]
    pub r#back_item_visual_identity_key: Option<usize>,
    #[serde(rename = "MainHand_ItemClassesKey")]
    pub r#main_hand_item_classes_key: Option<usize>,
    #[serde(rename = "OffHand_ItemClassesKey")]
    pub r#off_hand_item_classes_key: Option<usize>,
    #[serde(rename = "Helmet_ItemVisualIdentityKey")]
    pub r#helmet_item_visual_identity_key: Option<usize>,
    #[serde(rename = "KillSpecificMonsterCount_AchievementItemsKeys")]
    pub r#kill_specific_monster_count_achievement_items_keys: Vec<usize>,
    #[serde(rename = "Special_ModsKeys")]
    pub r#special_mods_keys: Vec<usize>,
    #[serde(rename = "KillRare_AchievementItemsKeys")]
    pub r#kill_rare_achievement_items_keys: Vec<usize>,
    #[serde(rename = "HASH16")]
    pub r#hash16: i16,
    #[serde(rename = "KillWhileOnslaughtIsActive_AchievementItemsKey")]
    pub r#kill_while_onslaught_is_active_achievement_items_key: Option<usize>,
    #[serde(rename = "MonsterSegmentsKey")]
    pub r#monster_segments_key: Option<usize>,
    #[serde(rename = "MonsterArmoursKey")]
    pub r#monster_armours_key: Option<usize>,
    #[serde(rename = "KillWhileTalismanIsActive_AchievementItemsKey")]
    pub r#kill_while_talisman_is_active_achievement_items_key: Option<usize>,
    #[serde(rename = "Part1_ModsKeys")]
    pub r#part1_mods_keys: Vec<usize>,
    #[serde(rename = "Part2_ModsKeys")]
    pub r#part2_mods_keys: Vec<usize>,
    #[serde(rename = "Endgame_ModsKeys")]
    pub r#endgame_mods_keys: Vec<usize>,
    #[serde(rename = "SinkAnimation_AOFile")]
    pub r#sink_animation_ao_file: String,
    #[serde(rename = "EPKFile")]
    pub r#epk_file: String,
    #[serde(rename = "MonsterConditionalEffectPacksKey")]
    pub r#monster_conditional_effect_packs_key: Option<usize>,
    #[serde(rename = "AddonMonsterTypeIndex")]
    pub r#addon_monster_type_index: Vec<String>,
    #[serde(rename = "BossHealthBar")]
    pub r#boss_health_bar: bool,
}
