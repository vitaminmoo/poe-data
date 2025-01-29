#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct ItemClasses {
    pub r#id: String,
    pub r#name: String,
    pub r#trade_market_category: Option<TradeMarketCategoryRow>,
    pub r#item_class_category: Option<ItemClassCategoriesRow>,
    pub r#removed_if_leaves_area: bool,
    pub r#identify_achievements: Vec<AchievementItemsRow>,
    pub r#allocate_to_map_owner: bool,
    pub r#always_allocate: bool,
    pub r#can_have_veiled_mods: bool,
    pub r#picked_up_quest: Option<QuestFlagsRow>,
    pub r#always_show: bool,
    pub r#can_be_corrupted: bool,
    pub r#can_have_incubators: bool,
    pub r#can_have_influence: bool,
    pub r#can_be_double_corrupted: bool,
    pub r#can_have_aspects: bool,
    pub r#can_transfer_skin: bool,
    pub r#item_stance: Option<ItemStancesRow>,
    pub r#can_scourge: bool,
    pub r#can_upgrade_rarity: bool,
    pub r#max_inventory_dimensions: Vec<i32>,
    pub r#flags: Vec<ItemClassFlags>,
    pub r#unmodifiable: bool,
    pub r#can_be_fractured: bool,
    pub r#equip_achievements: Option<AchievementItemsRow>,
    pub r#used_in_map_device: bool,
}

#[allow(non_upper_case_globals)]
pub static TABLE_ItemClasses: LazyLock<Vec<ItemClasses>> = LazyLock::new(|| {
    RAW_TABLE_ItemClasses
        .iter()
        .map(|x| ItemClasses {
            r#id: x.r#id.clone(),
            r#name: x.r#name.clone(),
            r#trade_market_category: x.r#trade_market_category.map(TradeMarketCategoryRow),
            r#item_class_category: x.r#item_class_category.map(ItemClassCategoriesRow),
            r#removed_if_leaves_area: x.r#removed_if_leaves_area.clone(),
            r#identify_achievements: x
                .r#identify_achievements
                .iter()
                .copied()
                .map(AchievementItemsRow)
                .collect(),
            r#allocate_to_map_owner: x.r#allocate_to_map_owner.clone(),
            r#always_allocate: x.r#always_allocate.clone(),
            r#can_have_veiled_mods: x.r#can_have_veiled_mods.clone(),
            r#picked_up_quest: x.r#picked_up_quest.map(QuestFlagsRow),
            r#always_show: x.r#always_show.clone(),
            r#can_be_corrupted: x.r#can_be_corrupted.clone(),
            r#can_have_incubators: x.r#can_have_incubators.clone(),
            r#can_have_influence: x.r#can_have_influence.clone(),
            r#can_be_double_corrupted: x.r#can_be_double_corrupted.clone(),
            r#can_have_aspects: x.r#can_have_aspects.clone(),
            r#can_transfer_skin: x.r#can_transfer_skin.clone(),
            r#item_stance: x.r#item_stance.map(ItemStancesRow),
            r#can_scourge: x.r#can_scourge.clone(),
            r#can_upgrade_rarity: x.r#can_upgrade_rarity.clone(),
            r#max_inventory_dimensions: x.r#max_inventory_dimensions.clone(),
            r#flags: x
                .r#flags
                .iter()
                .map(|y| ItemClassFlags::from_repr(*y).unwrap())
                .collect(),
            r#unmodifiable: x.r#unmodifiable.clone(),
            r#can_be_fractured: x.r#can_be_fractured.clone(),
            r#equip_achievements: x.r#equip_achievements.map(AchievementItemsRow),
            r#used_in_map_device: x.r#used_in_map_device.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ItemClassesRow(pub usize);

impl Deref for ItemClassesRow {
    type Target = ItemClasses;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_ItemClasses[self.0]
    }
}

impl ItemClassesRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ItemClasses {
        &TABLE_ItemClasses[self.0]
    }
    pub fn get(&self) -> &'static ItemClasses {
        &TABLE_ItemClasses[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ItemClasses.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ItemClasses)> {
        TABLE_ItemClasses
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_ItemClasses: LazyLock<Vec<ItemClassesRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/ItemClasses.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct ItemClassesRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "Name")]
    pub r#name: String,
    #[serde(rename = "TradeMarketCategory")]
    pub r#trade_market_category: Option<usize>,
    #[serde(rename = "ItemClassCategory")]
    pub r#item_class_category: Option<usize>,
    #[serde(rename = "RemovedIfLeavesArea")]
    pub r#removed_if_leaves_area: bool,
    #[serde(rename = "IdentifyAchievements")]
    pub r#identify_achievements: Vec<usize>,
    #[serde(rename = "AllocateToMapOwner")]
    pub r#allocate_to_map_owner: bool,
    #[serde(rename = "AlwaysAllocate")]
    pub r#always_allocate: bool,
    #[serde(rename = "CanHaveVeiledMods")]
    pub r#can_have_veiled_mods: bool,
    #[serde(rename = "PickedUpQuest")]
    pub r#picked_up_quest: Option<usize>,
    #[serde(rename = "AlwaysShow")]
    pub r#always_show: bool,
    #[serde(rename = "CanBeCorrupted")]
    pub r#can_be_corrupted: bool,
    #[serde(rename = "CanHaveIncubators")]
    pub r#can_have_incubators: bool,
    #[serde(rename = "CanHaveInfluence")]
    pub r#can_have_influence: bool,
    #[serde(rename = "CanBeDoubleCorrupted")]
    pub r#can_be_double_corrupted: bool,
    #[serde(rename = "CanHaveAspects")]
    pub r#can_have_aspects: bool,
    #[serde(rename = "CanTransferSkin")]
    pub r#can_transfer_skin: bool,
    #[serde(rename = "ItemStance")]
    pub r#item_stance: Option<usize>,
    #[serde(rename = "CanScourge")]
    pub r#can_scourge: bool,
    #[serde(rename = "CanUpgradeRarity")]
    pub r#can_upgrade_rarity: bool,
    #[serde(rename = "MaxInventoryDimensions")]
    pub r#max_inventory_dimensions: Vec<i32>,
    #[serde(rename = "Flags")]
    pub r#flags: Vec<usize>,
    #[serde(rename = "Unmodifiable")]
    pub r#unmodifiable: bool,
    #[serde(rename = "CanBeFractured")]
    pub r#can_be_fractured: bool,
    #[serde(rename = "EquipAchievements")]
    pub r#equip_achievements: Option<usize>,
    #[serde(rename = "UsedInMapDevice")]
    pub r#used_in_map_device: bool,
}
