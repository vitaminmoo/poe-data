#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct QuestFlags {
    pub r#id: String,
    pub r#hash32: i32,
}

#[allow(non_upper_case_globals)]
pub static TABLE_QuestFlags: LazyLock<Vec<QuestFlags>> = LazyLock::new(|| {
    RAW_TABLE_QuestFlags
        .iter()
        .map(|x| QuestFlags {
            r#id: x.r#id.clone(),
            r#hash32: x.r#hash32.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct QuestFlagsRow(pub usize);

impl Deref for QuestFlagsRow {
    type Target = QuestFlags;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_QuestFlags[self.0]
    }
}

impl QuestFlagsRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static QuestFlags {
        &TABLE_QuestFlags[self.0]
    }
    pub fn get(&self) -> &'static QuestFlags {
        &TABLE_QuestFlags[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_QuestFlags.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static QuestFlags)> {
        TABLE_QuestFlags
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_QuestFlags: LazyLock<Vec<QuestFlagsRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/QuestFlags.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct QuestFlagsRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "HASH32")]
    pub r#hash32: i32,
}
