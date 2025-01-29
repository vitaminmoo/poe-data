#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct HideoutDoodadTags {
    pub r#id: String,
    pub r#name: String,
}

#[allow(non_upper_case_globals)]
pub static TABLE_HideoutDoodadTags: LazyLock<Vec<HideoutDoodadTags>> = LazyLock::new(|| {
    RAW_TABLE_HideoutDoodadTags
        .iter()
        .map(|x| HideoutDoodadTags {
            r#id: x.r#id.clone(),
            r#name: x.r#name.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct HideoutDoodadTagsRow(pub usize);

impl Deref for HideoutDoodadTagsRow {
    type Target = HideoutDoodadTags;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_HideoutDoodadTags[self.0]
    }
}

impl HideoutDoodadTagsRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static HideoutDoodadTags {
        &TABLE_HideoutDoodadTags[self.0]
    }
    pub fn get(&self) -> &'static HideoutDoodadTags {
        &TABLE_HideoutDoodadTags[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_HideoutDoodadTags
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static HideoutDoodadTags)> {
        TABLE_HideoutDoodadTags
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_HideoutDoodadTags: LazyLock<Vec<HideoutDoodadTagsRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/HideoutDoodadTags.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct HideoutDoodadTagsRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "Name")]
    pub r#name: String,
}
