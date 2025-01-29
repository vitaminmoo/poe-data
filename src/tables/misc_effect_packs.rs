#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct MiscEffectPacks {
    pub r#id: String,
    pub r#epk_file: String,
    pub r#preload_groups: Vec<PreloadGroupsRow>,
    pub r#player_only_epk_file: String,
}

#[allow(non_upper_case_globals)]
pub static TABLE_MiscEffectPacks: LazyLock<Vec<MiscEffectPacks>> = LazyLock::new(|| {
    RAW_TABLE_MiscEffectPacks
        .iter()
        .map(|x| MiscEffectPacks {
            r#id: x.r#id.clone(),
            r#epk_file: x.r#epk_file.clone(),
            r#preload_groups: x
                .r#preload_groups
                .iter()
                .copied()
                .map(PreloadGroupsRow)
                .collect(),
            r#player_only_epk_file: x.r#player_only_epk_file.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MiscEffectPacksRow(pub usize);

impl Deref for MiscEffectPacksRow {
    type Target = MiscEffectPacks;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_MiscEffectPacks[self.0]
    }
}

impl MiscEffectPacksRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MiscEffectPacks {
        &TABLE_MiscEffectPacks[self.0]
    }
    pub fn get(&self) -> &'static MiscEffectPacks {
        &TABLE_MiscEffectPacks[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MiscEffectPacks
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MiscEffectPacks)> {
        TABLE_MiscEffectPacks
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_MiscEffectPacks: LazyLock<Vec<MiscEffectPacksRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/MiscEffectPacks.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct MiscEffectPacksRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "EPKFile")]
    pub r#epk_file: String,
    #[serde(rename = "PreloadGroups")]
    pub r#preload_groups: Vec<usize>,
    #[serde(rename = "PlayerOnly_EPKFile")]
    pub r#player_only_epk_file: String,
}
