#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct NPCs {
    pub r#id: String,
    pub r#name: String,
    pub r#metadata: String,
    pub r#npc_master_key: Option<NPCMasterRow>,
    pub r#short_name: String,
    pub r#npc_audios1: Vec<NPCAudioRow>,
    pub r#npc_audios2: Vec<NPCAudioRow>,
    pub r#hash16: i16,
    pub r#portrait: Option<NPCPortraitsRow>,
    pub r#dialogue_style: Option<NPCDialogueStylesRow>,
    pub r#gender: String,
}

#[allow(non_upper_case_globals)]
pub static TABLE_NPCs: LazyLock<Vec<NPCs>> = LazyLock::new(|| {
    RAW_TABLE_NPCs
        .iter()
        .map(|x| NPCs {
            r#id: x.r#id.clone(),
            r#name: x.r#name.clone(),
            r#metadata: x.r#metadata.clone(),
            r#npc_master_key: x.r#npc_master_key.map(NPCMasterRow),
            r#short_name: x.r#short_name.clone(),
            r#npc_audios1: x.r#npc_audios1.iter().copied().map(NPCAudioRow).collect(),
            r#npc_audios2: x.r#npc_audios2.iter().copied().map(NPCAudioRow).collect(),
            r#hash16: x.r#hash16.clone(),
            r#portrait: x.r#portrait.map(NPCPortraitsRow),
            r#dialogue_style: x.r#dialogue_style.map(NPCDialogueStylesRow),
            r#gender: x.r#gender.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct NPCsRow(pub usize);

impl Deref for NPCsRow {
    type Target = NPCs;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_NPCs[self.0]
    }
}

impl NPCsRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static NPCs {
        &TABLE_NPCs[self.0]
    }
    pub fn get(&self) -> &'static NPCs {
        &TABLE_NPCs[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_NPCs.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static NPCs)> {
        TABLE_NPCs.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_NPCs: LazyLock<Vec<NPCsRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/NPCs.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct NPCsRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "Name")]
    pub r#name: String,
    #[serde(rename = "Metadata")]
    pub r#metadata: String,
    #[serde(rename = "NPCMasterKey")]
    pub r#npc_master_key: Option<usize>,
    #[serde(rename = "ShortName")]
    pub r#short_name: String,
    #[serde(rename = "NPCAudios1")]
    pub r#npc_audios1: Vec<usize>,
    #[serde(rename = "NPCAudios2")]
    pub r#npc_audios2: Vec<usize>,
    #[serde(rename = "HASH16")]
    pub r#hash16: i16,
    #[serde(rename = "Portrait")]
    pub r#portrait: Option<usize>,
    #[serde(rename = "DialogueStyle")]
    pub r#dialogue_style: Option<usize>,
    #[serde(rename = "Gender")]
    pub r#gender: String,
}
