#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct BaseItemTypes {
    pub r#id: String,
    pub r#item_classes_key: Option<ItemClassesRow>,
    pub r#width: i32,
    pub r#height: i32,
    pub r#name: String,
    pub r#inherits_from: String,
    pub r#drop_level: i32,
    pub r#flavour_text_key: Option<FlavourTextRow>,
    pub r#implicit_mods_keys: Vec<ModsRow>,
    pub r#size_on_ground: i32,
    pub r#sound_effect: Option<SoundEffectsRow>,
    pub r#tags_keys: Vec<TagsRow>,
    pub r#mod_domain: MaybeVariant<ModDomains>,
    pub r#site_visibility: i32,
    pub r#item_visual_identity: Option<ItemVisualIdentityRow>,
    pub r#hash32: i32,
    pub r#vendor_recipe_achievement_items: Vec<AchievementItemsRow>,
    pub r#inflection: String,
    pub r#equip_achievement_items_key: Option<AchievementItemsRow>,
    pub r#is_corrupted: bool,
    pub r#identify_achievement_items: Vec<AchievementItemsRow>,
    pub r#identify_magic_achievement_items: Vec<AchievementItemsRow>,
    pub r#fragment_base_item_types_key: Option<BaseItemTypesRow>,
    pub r#trade_market_category: Option<TradeMarketCategoryRow>,
    pub r#unmodifiable: bool,
    pub r#achievement: Vec<AchievementItemsRow>,
    pub r#ignore_quant_bonus: bool,
}

#[allow(non_upper_case_globals)]
pub static TABLE_BaseItemTypes: LazyLock<Vec<BaseItemTypes>> = LazyLock::new(|| {
    RAW_TABLE_BaseItemTypes
        .iter()
        .map(|x| BaseItemTypes {
            r#id: x.r#id.clone(),
            r#item_classes_key: x.r#item_classes_key.map(ItemClassesRow),
            r#width: x.r#width.clone(),
            r#height: x.r#height.clone(),
            r#name: x.r#name.clone(),
            r#inherits_from: x.r#inherits_from.clone(),
            r#drop_level: x.r#drop_level.clone(),
            r#flavour_text_key: x.r#flavour_text_key.map(FlavourTextRow),
            r#implicit_mods_keys: x
                .r#implicit_mods_keys
                .iter()
                .copied()
                .map(ModsRow)
                .collect(),
            r#size_on_ground: x.r#size_on_ground.clone(),
            r#sound_effect: x.r#sound_effect.map(SoundEffectsRow),
            r#tags_keys: x.r#tags_keys.iter().copied().map(TagsRow).collect(),
            r#mod_domain: ModDomains::from_repr(x.r#mod_domain).map_or(
                MaybeVariant::NotVariant(x.r#mod_domain),
                MaybeVariant::Variant,
            ),
            r#site_visibility: x.r#site_visibility.clone(),
            r#item_visual_identity: x.r#item_visual_identity.map(ItemVisualIdentityRow),
            r#hash32: x.r#hash32.clone(),
            r#vendor_recipe_achievement_items: x
                .r#vendor_recipe_achievement_items
                .iter()
                .copied()
                .map(AchievementItemsRow)
                .collect(),
            r#inflection: x.r#inflection.clone(),
            r#equip_achievement_items_key: x.r#equip_achievement_items_key.map(AchievementItemsRow),
            r#is_corrupted: x.r#is_corrupted.clone(),
            r#identify_achievement_items: x
                .r#identify_achievement_items
                .iter()
                .copied()
                .map(AchievementItemsRow)
                .collect(),
            r#identify_magic_achievement_items: x
                .r#identify_magic_achievement_items
                .iter()
                .copied()
                .map(AchievementItemsRow)
                .collect(),
            r#fragment_base_item_types_key: x.r#fragment_base_item_types_key.map(BaseItemTypesRow),
            r#trade_market_category: x.r#trade_market_category.map(TradeMarketCategoryRow),
            r#unmodifiable: x.r#unmodifiable.clone(),
            r#achievement: x
                .r#achievement
                .iter()
                .copied()
                .map(AchievementItemsRow)
                .collect(),
            r#ignore_quant_bonus: x.r#ignore_quant_bonus.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BaseItemTypesRow(pub usize);

impl Deref for BaseItemTypesRow {
    type Target = BaseItemTypes;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_BaseItemTypes[self.0]
    }
}

impl BaseItemTypesRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BaseItemTypes {
        &TABLE_BaseItemTypes[self.0]
    }
    pub fn get(&self) -> &'static BaseItemTypes {
        &TABLE_BaseItemTypes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BaseItemTypes.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BaseItemTypes)> {
        TABLE_BaseItemTypes
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_BaseItemTypes: LazyLock<Vec<BaseItemTypesRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/BaseItemTypes.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct BaseItemTypesRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "ItemClassesKey")]
    pub r#item_classes_key: Option<usize>,
    #[serde(rename = "Width")]
    pub r#width: i32,
    #[serde(rename = "Height")]
    pub r#height: i32,
    #[serde(rename = "Name")]
    pub r#name: String,
    #[serde(rename = "InheritsFrom")]
    pub r#inherits_from: String,
    #[serde(rename = "DropLevel")]
    pub r#drop_level: i32,
    #[serde(rename = "FlavourTextKey")]
    pub r#flavour_text_key: Option<usize>,
    #[serde(rename = "Implicit_ModsKeys")]
    pub r#implicit_mods_keys: Vec<usize>,
    #[serde(rename = "SizeOnGround")]
    pub r#size_on_ground: i32,
    #[serde(rename = "SoundEffect")]
    pub r#sound_effect: Option<usize>,
    #[serde(rename = "TagsKeys")]
    pub r#tags_keys: Vec<usize>,
    #[serde(rename = "ModDomain")]
    pub r#mod_domain: usize,
    #[serde(rename = "SiteVisibility")]
    pub r#site_visibility: i32,
    #[serde(rename = "ItemVisualIdentity")]
    pub r#item_visual_identity: Option<usize>,
    #[serde(rename = "HASH32")]
    pub r#hash32: i32,
    #[serde(rename = "VendorRecipe_AchievementItems")]
    pub r#vendor_recipe_achievement_items: Vec<usize>,
    #[serde(rename = "Inflection")]
    pub r#inflection: String,
    #[serde(rename = "Equip_AchievementItemsKey")]
    pub r#equip_achievement_items_key: Option<usize>,
    #[serde(rename = "IsCorrupted")]
    pub r#is_corrupted: bool,
    #[serde(rename = "Identify_AchievementItems")]
    pub r#identify_achievement_items: Vec<usize>,
    #[serde(rename = "IdentifyMagic_AchievementItems")]
    pub r#identify_magic_achievement_items: Vec<usize>,
    #[serde(rename = "FragmentBaseItemTypesKey")]
    pub r#fragment_base_item_types_key: Option<usize>,
    #[serde(rename = "TradeMarketCategory")]
    pub r#trade_market_category: Option<usize>,
    #[serde(rename = "Unmodifiable")]
    pub r#unmodifiable: bool,
    #[serde(rename = "Achievement")]
    pub r#achievement: Vec<usize>,
    #[serde(rename = "IgnoreQuantBonus")]
    pub r#ignore_quant_bonus: bool,
}
