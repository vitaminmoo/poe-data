#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct HideoutDoodads {
    pub r#base_item_types_key: Option<BaseItemTypesRow>,
    pub r#variation_ao_files: Vec<String>,
    pub r#is_non_master_doodad: bool,
    pub r#inherits_from: String,
    pub r#is_crafting_bench: bool,
    pub r#tags: Vec<HideoutDoodadTagsRow>,
    pub r#category: Option<HideoutDoodadCategoryRow>,
}

#[allow(non_upper_case_globals)]
pub static TABLE_HideoutDoodads: LazyLock<Vec<HideoutDoodads>> = LazyLock::new(|| {
    RAW_TABLE_HideoutDoodads
        .iter()
        .map(|x| HideoutDoodads {
            r#base_item_types_key: x.r#base_item_types_key.map(BaseItemTypesRow),
            r#variation_ao_files: x.r#variation_ao_files.clone(),
            r#is_non_master_doodad: x.r#is_non_master_doodad.clone(),
            r#inherits_from: x.r#inherits_from.clone(),
            r#is_crafting_bench: x.r#is_crafting_bench.clone(),
            r#tags: x.r#tags.iter().copied().map(HideoutDoodadTagsRow).collect(),
            r#category: x.r#category.map(HideoutDoodadCategoryRow),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct HideoutDoodadsRow(pub usize);

impl Deref for HideoutDoodadsRow {
    type Target = HideoutDoodads;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_HideoutDoodads[self.0]
    }
}

impl HideoutDoodadsRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static HideoutDoodads {
        &TABLE_HideoutDoodads[self.0]
    }
    pub fn get(&self) -> &'static HideoutDoodads {
        &TABLE_HideoutDoodads[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_HideoutDoodads
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static HideoutDoodads)> {
        TABLE_HideoutDoodads
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_HideoutDoodads: LazyLock<Vec<HideoutDoodadsRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/HideoutDoodads.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct HideoutDoodadsRaw {
    #[serde(rename = "BaseItemTypesKey")]
    pub r#base_item_types_key: Option<usize>,
    #[serde(rename = "Variation_AOFiles")]
    pub r#variation_ao_files: Vec<String>,
    #[serde(rename = "IsNonMasterDoodad")]
    pub r#is_non_master_doodad: bool,
    #[serde(rename = "InheritsFrom")]
    pub r#inherits_from: String,
    #[serde(rename = "IsCraftingBench")]
    pub r#is_crafting_bench: bool,
    #[serde(rename = "Tags")]
    pub r#tags: Vec<usize>,
    #[serde(rename = "Category")]
    pub r#category: Option<usize>,
}
