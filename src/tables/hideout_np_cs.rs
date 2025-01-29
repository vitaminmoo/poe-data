#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct HideoutNPCs {
    pub r#hideout_np_cs_key: Option<NPCsRow>,
    pub r#regular_np_cs_keys: Vec<NPCsRow>,
    pub r#hideout_doodads_key: Option<HideoutDoodadsRow>,
    pub r#npc_master_key: i32,
}

#[allow(non_upper_case_globals)]
pub static TABLE_HideoutNPCs: LazyLock<Vec<HideoutNPCs>> = LazyLock::new(|| {
    RAW_TABLE_HideoutNPCs
        .iter()
        .map(|x| HideoutNPCs {
            r#hideout_np_cs_key: x.r#hideout_np_cs_key.map(NPCsRow),
            r#regular_np_cs_keys: x
                .r#regular_np_cs_keys
                .iter()
                .copied()
                .map(NPCsRow)
                .collect(),
            r#hideout_doodads_key: x.r#hideout_doodads_key.map(HideoutDoodadsRow),
            r#npc_master_key: x.r#npc_master_key.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct HideoutNPCsRow(pub usize);

impl Deref for HideoutNPCsRow {
    type Target = HideoutNPCs;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_HideoutNPCs[self.0]
    }
}

impl HideoutNPCsRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static HideoutNPCs {
        &TABLE_HideoutNPCs[self.0]
    }
    pub fn get(&self) -> &'static HideoutNPCs {
        &TABLE_HideoutNPCs[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_HideoutNPCs.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static HideoutNPCs)> {
        TABLE_HideoutNPCs
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_HideoutNPCs: LazyLock<Vec<HideoutNPCsRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/HideoutNPCs.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct HideoutNPCsRaw {
    #[serde(rename = "Hideout_NPCsKey")]
    pub r#hideout_np_cs_key: Option<usize>,
    #[serde(rename = "Regular_NPCsKeys")]
    pub r#regular_np_cs_keys: Vec<usize>,
    #[serde(rename = "HideoutDoodadsKey")]
    pub r#hideout_doodads_key: Option<usize>,
    #[serde(rename = "NPCMasterKey")]
    pub r#npc_master_key: i32,
}
