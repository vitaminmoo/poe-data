#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct NPCPortraits {
    pub r#name: String,
    pub r#portrait_file: String,
}

#[allow(non_upper_case_globals)]
pub static TABLE_NPCPortraits: LazyLock<Vec<NPCPortraits>> = LazyLock::new(|| {
    RAW_TABLE_NPCPortraits
        .iter()
        .map(|x| NPCPortraits {
            r#name: x.r#name.clone(),
            r#portrait_file: x.r#portrait_file.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct NPCPortraitsRow(pub usize);

impl Deref for NPCPortraitsRow {
    type Target = NPCPortraits;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_NPCPortraits[self.0]
    }
}

impl NPCPortraitsRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static NPCPortraits {
        &TABLE_NPCPortraits[self.0]
    }
    pub fn get(&self) -> &'static NPCPortraits {
        &TABLE_NPCPortraits[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_NPCPortraits.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static NPCPortraits)> {
        TABLE_NPCPortraits
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_NPCPortraits: LazyLock<Vec<NPCPortraitsRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/NPCPortraits.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct NPCPortraitsRaw {
    #[serde(rename = "Name")]
    pub r#name: String,
    #[serde(rename = "PortraitFile")]
    pub r#portrait_file: String,
}
