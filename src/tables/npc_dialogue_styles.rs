#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct NPCDialogueStyles {
    pub r#id: String,
    pub r#header_base_file: String,
    pub r#buttom_file: String,
    pub r#banner_files: Vec<String>,
    pub r#header_files: Vec<String>,
}

#[allow(non_upper_case_globals)]
pub static TABLE_NPCDialogueStyles: LazyLock<Vec<NPCDialogueStyles>> = LazyLock::new(|| {
    RAW_TABLE_NPCDialogueStyles
        .iter()
        .map(|x| NPCDialogueStyles {
            r#id: x.r#id.clone(),
            r#header_base_file: x.r#header_base_file.clone(),
            r#buttom_file: x.r#buttom_file.clone(),
            r#banner_files: x.r#banner_files.clone(),
            r#header_files: x.r#header_files.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct NPCDialogueStylesRow(pub usize);

impl Deref for NPCDialogueStylesRow {
    type Target = NPCDialogueStyles;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_NPCDialogueStyles[self.0]
    }
}

impl NPCDialogueStylesRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static NPCDialogueStyles {
        &TABLE_NPCDialogueStyles[self.0]
    }
    pub fn get(&self) -> &'static NPCDialogueStyles {
        &TABLE_NPCDialogueStyles[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_NPCDialogueStyles
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static NPCDialogueStyles)> {
        TABLE_NPCDialogueStyles
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_NPCDialogueStyles: LazyLock<Vec<NPCDialogueStylesRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/NPCDialogueStyles.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct NPCDialogueStylesRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "HeaderBaseFile")]
    pub r#header_base_file: String,
    #[serde(rename = "ButtomFile")]
    pub r#buttom_file: String,
    #[serde(rename = "BannerFiles")]
    pub r#banner_files: Vec<String>,
    #[serde(rename = "HeaderFiles")]
    pub r#header_files: Vec<String>,
}
