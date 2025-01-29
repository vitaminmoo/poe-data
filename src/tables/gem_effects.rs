#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct GemEffects {
    pub r#id: String,
    pub r#name: String,
    pub r#granted_effect: Option<GrantedEffectsRow>,
    pub r#granted_effect_hardmode: Option<GrantedEffectsRow>,
    pub r#granted_effect2: Option<GrantedEffectsRow>,
    pub r#granted_effect2_hardmode: Option<GrantedEffectsRow>,
    pub r#support_text: String,
    pub r#support_name: String,
    pub r#gem_tags: Vec<GemTagsRow>,
    pub r#consumed_mods_key: Option<ModsRow>,
    pub r#item_color: i32,
}

#[allow(non_upper_case_globals)]
pub static TABLE_GemEffects: LazyLock<Vec<GemEffects>> = LazyLock::new(|| {
    RAW_TABLE_GemEffects
        .iter()
        .map(|x| GemEffects {
            r#id: x.r#id.clone(),
            r#name: x.r#name.clone(),
            r#granted_effect: x.r#granted_effect.map(GrantedEffectsRow),
            r#granted_effect_hardmode: x.r#granted_effect_hardmode.map(GrantedEffectsRow),
            r#granted_effect2: x.r#granted_effect2.map(GrantedEffectsRow),
            r#granted_effect2_hardmode: x.r#granted_effect2_hardmode.map(GrantedEffectsRow),
            r#support_text: x.r#support_text.clone(),
            r#support_name: x.r#support_name.clone(),
            r#gem_tags: x.r#gem_tags.iter().copied().map(GemTagsRow).collect(),
            r#consumed_mods_key: x.r#consumed_mods_key.map(ModsRow),
            r#item_color: x.r#item_color.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct GemEffectsRow(pub usize);

impl Deref for GemEffectsRow {
    type Target = GemEffects;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_GemEffects[self.0]
    }
}

impl GemEffectsRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static GemEffects {
        &TABLE_GemEffects[self.0]
    }
    pub fn get(&self) -> &'static GemEffects {
        &TABLE_GemEffects[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_GemEffects.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static GemEffects)> {
        TABLE_GemEffects
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_GemEffects: LazyLock<Vec<GemEffectsRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/GemEffects.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct GemEffectsRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "Name")]
    pub r#name: String,
    #[serde(rename = "GrantedEffect")]
    pub r#granted_effect: Option<usize>,
    #[serde(rename = "GrantedEffectHardmode")]
    pub r#granted_effect_hardmode: Option<usize>,
    #[serde(rename = "GrantedEffect2")]
    pub r#granted_effect2: Option<usize>,
    #[serde(rename = "GrantedEffect2Hardmode")]
    pub r#granted_effect2_hardmode: Option<usize>,
    #[serde(rename = "SupportText")]
    pub r#support_text: String,
    #[serde(rename = "SupportName")]
    pub r#support_name: String,
    #[serde(rename = "GemTags")]
    pub r#gem_tags: Vec<usize>,
    #[serde(rename = "Consumed_ModsKey")]
    pub r#consumed_mods_key: Option<usize>,
    #[serde(rename = "ItemColor")]
    pub r#item_color: i32,
}
