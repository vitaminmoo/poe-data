#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct BuffVisuals {
    pub r#id: String,
    pub r#buff_dds_file: String,
    pub r#epk_files1: Vec<String>,
    pub r#epk_files2: Vec<String>,
    pub r#preload_groups: Vec<PreloadGroupsRow>,
    pub r#buff_name: String,
    pub r#misc_animated1: Option<MiscAnimatedRow>,
    pub r#misc_animated2: Option<MiscAnimatedRow>,
    pub r#buff_description: String,
    pub r#epk_file: String,
    pub r#has_extra_art: bool,
    pub r#extra_art: String,
    pub r#epk_files: Vec<String>,
    pub r#buff_visual_orbs: Vec<BuffVisualOrbsRow>,
    pub r#misc_animated3: Option<MiscAnimatedRow>,
}

#[allow(non_upper_case_globals)]
pub static TABLE_BuffVisuals: LazyLock<Vec<BuffVisuals>> = LazyLock::new(|| {
    RAW_TABLE_BuffVisuals
        .iter()
        .map(|x| BuffVisuals {
            r#id: x.r#id.clone(),
            r#buff_dds_file: x.r#buff_dds_file.clone(),
            r#epk_files1: x.r#epk_files1.clone(),
            r#epk_files2: x.r#epk_files2.clone(),
            r#preload_groups: x
                .r#preload_groups
                .iter()
                .copied()
                .map(PreloadGroupsRow)
                .collect(),
            r#buff_name: x.r#buff_name.clone(),
            r#misc_animated1: x.r#misc_animated1.map(MiscAnimatedRow),
            r#misc_animated2: x.r#misc_animated2.map(MiscAnimatedRow),
            r#buff_description: x.r#buff_description.clone(),
            r#epk_file: x.r#epk_file.clone(),
            r#has_extra_art: x.r#has_extra_art.clone(),
            r#extra_art: x.r#extra_art.clone(),
            r#epk_files: x.r#epk_files.clone(),
            r#buff_visual_orbs: x
                .r#buff_visual_orbs
                .iter()
                .copied()
                .map(BuffVisualOrbsRow)
                .collect(),
            r#misc_animated3: x.r#misc_animated3.map(MiscAnimatedRow),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BuffVisualsRow(pub usize);

impl Deref for BuffVisualsRow {
    type Target = BuffVisuals;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_BuffVisuals[self.0]
    }
}

impl BuffVisualsRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BuffVisuals {
        &TABLE_BuffVisuals[self.0]
    }
    pub fn get(&self) -> &'static BuffVisuals {
        &TABLE_BuffVisuals[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BuffVisuals.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BuffVisuals)> {
        TABLE_BuffVisuals
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_BuffVisuals: LazyLock<Vec<BuffVisualsRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/BuffVisuals.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct BuffVisualsRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "BuffDDSFile")]
    pub r#buff_dds_file: String,
    #[serde(rename = "EPKFiles1")]
    pub r#epk_files1: Vec<String>,
    #[serde(rename = "EPKFiles2")]
    pub r#epk_files2: Vec<String>,
    #[serde(rename = "PreloadGroups")]
    pub r#preload_groups: Vec<usize>,
    #[serde(rename = "BuffName")]
    pub r#buff_name: String,
    #[serde(rename = "MiscAnimated1")]
    pub r#misc_animated1: Option<usize>,
    #[serde(rename = "MiscAnimated2")]
    pub r#misc_animated2: Option<usize>,
    #[serde(rename = "BuffDescription")]
    pub r#buff_description: String,
    #[serde(rename = "EPKFile")]
    pub r#epk_file: String,
    #[serde(rename = "HasExtraArt")]
    pub r#has_extra_art: bool,
    #[serde(rename = "ExtraArt")]
    pub r#extra_art: String,
    #[serde(rename = "EPKFiles")]
    pub r#epk_files: Vec<String>,
    #[serde(rename = "BuffVisualOrbs")]
    pub r#buff_visual_orbs: Vec<usize>,
    #[serde(rename = "MiscAnimated3")]
    pub r#misc_animated3: Option<usize>,
}
