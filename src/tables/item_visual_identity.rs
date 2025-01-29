#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct ItemVisualIdentity {
    pub r#id: String,
    pub r#dds_file: String,
    pub r#ao_file: String,
    pub r#inventory_sound_effect: Option<SoundEffectsRow>,
    pub r#hash16: i16,
    pub r#ao_file2: String,
    pub r#marauder_sm_files: Vec<String>,
    pub r#ranger_sm_files: Vec<String>,
    pub r#witch_sm_files: Vec<String>,
    pub r#duelist_dex_sm_files: Vec<String>,
    pub r#templar_sm_files: Vec<String>,
    pub r#shadow_sm_files: Vec<String>,
    pub r#scion_sm_files: Vec<String>,
    pub r#marauder_shape: String,
    pub r#ranger_shape: String,
    pub r#witch_shape: String,
    pub r#duelist_shape: String,
    pub r#templar_shape: String,
    pub r#shadow_shape: String,
    pub r#scion_shape: String,
    pub r#pickup_achievement_items_keys: Vec<AchievementItemsRow>,
    pub r#sm_files: Vec<String>,
    pub r#identify_achievement_items_keys: Vec<AchievementItemsRow>,
    pub r#epk_file: String,
    pub r#corrupt_achievement_items_keys: Vec<AchievementItemsRow>,
    pub r#is_alternate_art: bool,
    pub r#create_corrupted_jewel_achievement_items_key: Option<AchievementItemsRow>,
    pub r#animation_location: String,
    pub r#is_atlas_of_worlds_map_icon: bool,
    pub r#is_tier16_icon: bool,
    pub r#composition: i32,
}

#[allow(non_upper_case_globals)]
pub static TABLE_ItemVisualIdentity: LazyLock<Vec<ItemVisualIdentity>> = LazyLock::new(|| {
    RAW_TABLE_ItemVisualIdentity
        .iter()
        .map(|x| ItemVisualIdentity {
            r#id: x.r#id.clone(),
            r#dds_file: x.r#dds_file.clone(),
            r#ao_file: x.r#ao_file.clone(),
            r#inventory_sound_effect: x.r#inventory_sound_effect.map(SoundEffectsRow),
            r#hash16: x.r#hash16.clone(),
            r#ao_file2: x.r#ao_file2.clone(),
            r#marauder_sm_files: x.r#marauder_sm_files.clone(),
            r#ranger_sm_files: x.r#ranger_sm_files.clone(),
            r#witch_sm_files: x.r#witch_sm_files.clone(),
            r#duelist_dex_sm_files: x.r#duelist_dex_sm_files.clone(),
            r#templar_sm_files: x.r#templar_sm_files.clone(),
            r#shadow_sm_files: x.r#shadow_sm_files.clone(),
            r#scion_sm_files: x.r#scion_sm_files.clone(),
            r#marauder_shape: x.r#marauder_shape.clone(),
            r#ranger_shape: x.r#ranger_shape.clone(),
            r#witch_shape: x.r#witch_shape.clone(),
            r#duelist_shape: x.r#duelist_shape.clone(),
            r#templar_shape: x.r#templar_shape.clone(),
            r#shadow_shape: x.r#shadow_shape.clone(),
            r#scion_shape: x.r#scion_shape.clone(),
            r#pickup_achievement_items_keys: x
                .r#pickup_achievement_items_keys
                .iter()
                .copied()
                .map(AchievementItemsRow)
                .collect(),
            r#sm_files: x.r#sm_files.clone(),
            r#identify_achievement_items_keys: x
                .r#identify_achievement_items_keys
                .iter()
                .copied()
                .map(AchievementItemsRow)
                .collect(),
            r#epk_file: x.r#epk_file.clone(),
            r#corrupt_achievement_items_keys: x
                .r#corrupt_achievement_items_keys
                .iter()
                .copied()
                .map(AchievementItemsRow)
                .collect(),
            r#is_alternate_art: x.r#is_alternate_art.clone(),
            r#create_corrupted_jewel_achievement_items_key: x
                .r#create_corrupted_jewel_achievement_items_key
                .map(AchievementItemsRow),
            r#animation_location: x.r#animation_location.clone(),
            r#is_atlas_of_worlds_map_icon: x.r#is_atlas_of_worlds_map_icon.clone(),
            r#is_tier16_icon: x.r#is_tier16_icon.clone(),
            r#composition: x.r#composition.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ItemVisualIdentityRow(pub usize);

impl Deref for ItemVisualIdentityRow {
    type Target = ItemVisualIdentity;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_ItemVisualIdentity[self.0]
    }
}

impl ItemVisualIdentityRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ItemVisualIdentity {
        &TABLE_ItemVisualIdentity[self.0]
    }
    pub fn get(&self) -> &'static ItemVisualIdentity {
        &TABLE_ItemVisualIdentity[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ItemVisualIdentity
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ItemVisualIdentity)> {
        TABLE_ItemVisualIdentity
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_ItemVisualIdentity: LazyLock<Vec<ItemVisualIdentityRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/ItemVisualIdentity.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct ItemVisualIdentityRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "DDSFile")]
    pub r#dds_file: String,
    #[serde(rename = "AOFile")]
    pub r#ao_file: String,
    #[serde(rename = "InventorySoundEffect")]
    pub r#inventory_sound_effect: Option<usize>,
    #[serde(rename = "HASH16")]
    pub r#hash16: i16,
    #[serde(rename = "AOFile2")]
    pub r#ao_file2: String,
    #[serde(rename = "MarauderSMFiles")]
    pub r#marauder_sm_files: Vec<String>,
    #[serde(rename = "RangerSMFiles")]
    pub r#ranger_sm_files: Vec<String>,
    #[serde(rename = "WitchSMFiles")]
    pub r#witch_sm_files: Vec<String>,
    #[serde(rename = "DuelistDexSMFiles")]
    pub r#duelist_dex_sm_files: Vec<String>,
    #[serde(rename = "TemplarSMFiles")]
    pub r#templar_sm_files: Vec<String>,
    #[serde(rename = "ShadowSMFiles")]
    pub r#shadow_sm_files: Vec<String>,
    #[serde(rename = "ScionSMFiles")]
    pub r#scion_sm_files: Vec<String>,
    #[serde(rename = "MarauderShape")]
    pub r#marauder_shape: String,
    #[serde(rename = "RangerShape")]
    pub r#ranger_shape: String,
    #[serde(rename = "WitchShape")]
    pub r#witch_shape: String,
    #[serde(rename = "DuelistShape")]
    pub r#duelist_shape: String,
    #[serde(rename = "TemplarShape")]
    pub r#templar_shape: String,
    #[serde(rename = "ShadowShape")]
    pub r#shadow_shape: String,
    #[serde(rename = "ScionShape")]
    pub r#scion_shape: String,
    #[serde(rename = "Pickup_AchievementItemsKeys")]
    pub r#pickup_achievement_items_keys: Vec<usize>,
    #[serde(rename = "SMFiles")]
    pub r#sm_files: Vec<String>,
    #[serde(rename = "Identify_AchievementItemsKeys")]
    pub r#identify_achievement_items_keys: Vec<usize>,
    #[serde(rename = "EPKFile")]
    pub r#epk_file: String,
    #[serde(rename = "Corrupt_AchievementItemsKeys")]
    pub r#corrupt_achievement_items_keys: Vec<usize>,
    #[serde(rename = "IsAlternateArt")]
    pub r#is_alternate_art: bool,
    #[serde(rename = "CreateCorruptedJewelAchievementItemsKey")]
    pub r#create_corrupted_jewel_achievement_items_key: Option<usize>,
    #[serde(rename = "AnimationLocation")]
    pub r#animation_location: String,
    #[serde(rename = "IsAtlasOfWorldsMapIcon")]
    pub r#is_atlas_of_worlds_map_icon: bool,
    #[serde(rename = "IsTier16Icon")]
    pub r#is_tier16_icon: bool,
    #[serde(rename = "Composition")]
    pub r#composition: i32,
}
