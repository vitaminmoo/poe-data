#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct GemTags {
    pub r#id: String,
    pub r#tag: String,
    pub r#stat1: Option<StatsRow>,
    pub r#stat2: Option<StatsRow>,
    pub r#stat3: Option<StatsRow>,
    pub r#stat4: Option<StatsRow>,
}

#[allow(non_upper_case_globals)]
pub static TABLE_GemTags: LazyLock<Vec<GemTags>> = LazyLock::new(|| {
    RAW_TABLE_GemTags
        .iter()
        .map(|x| GemTags {
            r#id: x.r#id.clone(),
            r#tag: x.r#tag.clone(),
            r#stat1: x.r#stat1.map(StatsRow),
            r#stat2: x.r#stat2.map(StatsRow),
            r#stat3: x.r#stat3.map(StatsRow),
            r#stat4: x.r#stat4.map(StatsRow),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct GemTagsRow(pub usize);

impl Deref for GemTagsRow {
    type Target = GemTags;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_GemTags[self.0]
    }
}

impl GemTagsRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static GemTags {
        &TABLE_GemTags[self.0]
    }
    pub fn get(&self) -> &'static GemTags {
        &TABLE_GemTags[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_GemTags.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static GemTags)> {
        TABLE_GemTags.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_GemTags: LazyLock<Vec<GemTagsRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/GemTags.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct GemTagsRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "Tag")]
    pub r#tag: String,
    #[serde(rename = "Stat1")]
    pub r#stat1: Option<usize>,
    #[serde(rename = "Stat2")]
    pub r#stat2: Option<usize>,
    #[serde(rename = "Stat3")]
    pub r#stat3: Option<usize>,
    #[serde(rename = "Stat4")]
    pub r#stat4: Option<usize>,
}
