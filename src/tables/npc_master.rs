#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct NPCMaster {
    pub r#id: String,
    pub r#signature_mods_key: Option<ModsRow>,
    pub r#spawn_weight_tags_keys: Vec<TagsRow>,
    pub r#spawn_weight_values: Vec<i32>,
    pub r#area_description: String,
    pub r#has_area_missions: bool,
}

#[allow(non_upper_case_globals)]
pub static TABLE_NPCMaster: LazyLock<Vec<NPCMaster>> = LazyLock::new(|| {
    RAW_TABLE_NPCMaster
        .iter()
        .map(|x| NPCMaster {
            r#id: x.r#id.clone(),
            r#signature_mods_key: x.r#signature_mods_key.map(ModsRow),
            r#spawn_weight_tags_keys: x
                .r#spawn_weight_tags_keys
                .iter()
                .copied()
                .map(TagsRow)
                .collect(),
            r#spawn_weight_values: x.r#spawn_weight_values.clone(),
            r#area_description: x.r#area_description.clone(),
            r#has_area_missions: x.r#has_area_missions.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct NPCMasterRow(pub usize);

impl Deref for NPCMasterRow {
    type Target = NPCMaster;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_NPCMaster[self.0]
    }
}

impl NPCMasterRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static NPCMaster {
        &TABLE_NPCMaster[self.0]
    }
    pub fn get(&self) -> &'static NPCMaster {
        &TABLE_NPCMaster[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_NPCMaster.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static NPCMaster)> {
        TABLE_NPCMaster
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_NPCMaster: LazyLock<Vec<NPCMasterRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/NPCMaster.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct NPCMasterRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "Signature_ModsKey")]
    pub r#signature_mods_key: Option<usize>,
    #[serde(rename = "SpawnWeight_TagsKeys")]
    pub r#spawn_weight_tags_keys: Vec<usize>,
    #[serde(rename = "SpawnWeight_Values")]
    pub r#spawn_weight_values: Vec<i32>,
    #[serde(rename = "AreaDescription")]
    pub r#area_description: String,
    #[serde(rename = "HasAreaMissions")]
    pub r#has_area_missions: bool,
}
