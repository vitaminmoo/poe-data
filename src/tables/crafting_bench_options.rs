#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct CraftingBenchOptions {
    pub r#hideout_np_cs_key: Option<HideoutNPCsRow>,
    pub r#order: i32,
    pub r#add_mod: Option<ModsRow>,
    pub r#cost_base_item_types: Vec<BaseItemTypesRow>,
    pub r#cost_values: Vec<i32>,
    pub r#required_level: i32,
    pub r#name: String,
    pub r#crafting_bench_custom_action: MaybeVariant<CraftingBenchCustomActions>,
    pub r#item_classes: Vec<ItemClassesRow>,
    pub r#links: i32,
    pub r#socket_colours: String,
    pub r#sockets: i32,
    pub r#item_quantity: i32,
    pub r#description: String,
    pub r#is_disabled: bool,
    pub r#is_area_option: bool,
    // column_ref_array recipe_ids
    pub r#tier: i32,
    pub r#crafting_item_class_categories: Vec<CraftingItemClassCategoriesRow>,
    pub r#unlock_category: Option<CraftingBenchUnlockCategoriesRow>,
    pub r#unveils_required: i32,
    pub r#unveils_required2: i32,
    pub r#add_enchantment: Option<ModsRow>,
    pub r#sort_category: Option<CraftingBenchSortCategoriesRow>,
    pub r#map_device: bool,
    pub r#tags: Vec<TagsRow>,
}

#[allow(non_upper_case_globals)]
pub static TABLE_CraftingBenchOptions: LazyLock<Vec<CraftingBenchOptions>> = LazyLock::new(|| {
    RAW_TABLE_CraftingBenchOptions
        .iter()
        .map(|x| {
            CraftingBenchOptions {
                r#hideout_np_cs_key: x.r#hideout_np_cs_key.map(HideoutNPCsRow),
                r#order: x.r#order.clone(),
                r#add_mod: x.r#add_mod.map(ModsRow),
                r#cost_base_item_types: x
                    .r#cost_base_item_types
                    .iter()
                    .copied()
                    .map(BaseItemTypesRow)
                    .collect(),
                r#cost_values: x.r#cost_values.clone(),
                r#required_level: x.r#required_level.clone(),
                r#name: x.r#name.clone(),
                r#crafting_bench_custom_action: CraftingBenchCustomActions::from_repr(
                    x.r#crafting_bench_custom_action,
                )
                .map_or(
                    MaybeVariant::NotVariant(x.r#crafting_bench_custom_action),
                    MaybeVariant::Variant,
                ),
                r#item_classes: x
                    .r#item_classes
                    .iter()
                    .copied()
                    .map(ItemClassesRow)
                    .collect(),
                r#links: x.r#links.clone(),
                r#socket_colours: x.r#socket_colours.clone(),
                r#sockets: x.r#sockets.clone(),
                r#item_quantity: x.r#item_quantity.clone(),
                r#description: x.r#description.clone(),
                r#is_disabled: x.r#is_disabled.clone(),
                r#is_area_option: x.r#is_area_option.clone(),
                // column_ref_array recipe_ids
                r#tier: x.r#tier.clone(),
                r#crafting_item_class_categories: x
                    .r#crafting_item_class_categories
                    .iter()
                    .copied()
                    .map(CraftingItemClassCategoriesRow)
                    .collect(),
                r#unlock_category: x.r#unlock_category.map(CraftingBenchUnlockCategoriesRow),
                r#unveils_required: x.r#unveils_required.clone(),
                r#unveils_required2: x.r#unveils_required2.clone(),
                r#add_enchantment: x.r#add_enchantment.map(ModsRow),
                r#sort_category: x.r#sort_category.map(CraftingBenchSortCategoriesRow),
                r#map_device: x.r#map_device.clone(),
                r#tags: x.r#tags.iter().copied().map(TagsRow).collect(),
            }
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct CraftingBenchOptionsRow(pub usize);

impl Deref for CraftingBenchOptionsRow {
    type Target = CraftingBenchOptions;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_CraftingBenchOptions[self.0]
    }
}

impl CraftingBenchOptionsRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static CraftingBenchOptions {
        &TABLE_CraftingBenchOptions[self.0]
    }
    pub fn get(&self) -> &'static CraftingBenchOptions {
        &TABLE_CraftingBenchOptions[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_CraftingBenchOptions
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static CraftingBenchOptions)> {
        TABLE_CraftingBenchOptions
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_CraftingBenchOptions: LazyLock<Vec<CraftingBenchOptionsRaw>> =
    LazyLock::new(|| {
        const DATA: &str = include_str!("../../data/tables/English/CraftingBenchOptions.json");
        serde_json::from_str(DATA).unwrap()
    });

#[derive(Debug, Deserialize, Serialize)]
struct CraftingBenchOptionsRaw {
    #[serde(rename = "HideoutNPCsKey")]
    pub r#hideout_np_cs_key: Option<usize>,
    #[serde(rename = "Order")]
    pub r#order: i32,
    #[serde(rename = "AddMod")]
    pub r#add_mod: Option<usize>,
    #[serde(rename = "Cost_BaseItemTypes")]
    pub r#cost_base_item_types: Vec<usize>,
    #[serde(rename = "Cost_Values")]
    pub r#cost_values: Vec<i32>,
    #[serde(rename = "RequiredLevel")]
    pub r#required_level: i32,
    #[serde(rename = "Name")]
    pub r#name: String,
    #[serde(rename = "CraftingBenchCustomAction")]
    pub r#crafting_bench_custom_action: usize,
    #[serde(rename = "ItemClasses")]
    pub r#item_classes: Vec<usize>,
    #[serde(rename = "Links")]
    pub r#links: i32,
    #[serde(rename = "SocketColours")]
    pub r#socket_colours: String,
    #[serde(rename = "Sockets")]
    pub r#sockets: i32,
    #[serde(rename = "ItemQuantity")]
    pub r#item_quantity: i32,
    #[serde(rename = "Description")]
    pub r#description: String,
    #[serde(rename = "IsDisabled")]
    pub r#is_disabled: bool,
    #[serde(rename = "IsAreaOption")]
    pub r#is_area_option: bool,
    #[serde(rename = "RecipeIds")]
    pub r#recipe_ids: Vec<i32>,
    #[serde(rename = "Tier")]
    pub r#tier: i32,
    #[serde(rename = "CraftingItemClassCategories")]
    pub r#crafting_item_class_categories: Vec<usize>,
    #[serde(rename = "UnlockCategory")]
    pub r#unlock_category: Option<usize>,
    #[serde(rename = "UnveilsRequired")]
    pub r#unveils_required: i32,
    #[serde(rename = "UnveilsRequired2")]
    pub r#unveils_required2: i32,
    #[serde(rename = "AddEnchantment")]
    pub r#add_enchantment: Option<usize>,
    #[serde(rename = "SortCategory")]
    pub r#sort_category: Option<usize>,
    #[serde(rename = "MapDevice")]
    pub r#map_device: bool,
    #[serde(rename = "Tags")]
    pub r#tags: Vec<usize>,
}
