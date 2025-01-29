#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct AchievementItems {
    pub r#id: String,
    pub r#name: String,
    pub r#completions_required: i32,
    pub r#achievements_key: Option<AchievementsRow>,
}

#[allow(non_upper_case_globals)]
pub static TABLE_AchievementItems: LazyLock<Vec<AchievementItems>> = LazyLock::new(|| {
    RAW_TABLE_AchievementItems
        .iter()
        .map(|x| AchievementItems {
            r#id: x.r#id.clone(),
            r#name: x.r#name.clone(),
            r#completions_required: x.r#completions_required.clone(),
            r#achievements_key: x.r#achievements_key.map(AchievementsRow),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct AchievementItemsRow(pub usize);

impl Deref for AchievementItemsRow {
    type Target = AchievementItems;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_AchievementItems[self.0]
    }
}

impl AchievementItemsRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static AchievementItems {
        &TABLE_AchievementItems[self.0]
    }
    pub fn get(&self) -> &'static AchievementItems {
        &TABLE_AchievementItems[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_AchievementItems
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static AchievementItems)> {
        TABLE_AchievementItems
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_AchievementItems: LazyLock<Vec<AchievementItemsRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/AchievementItems.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct AchievementItemsRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "Name")]
    pub r#name: String,
    #[serde(rename = "CompletionsRequired")]
    pub r#completions_required: i32,
    #[serde(rename = "AchievementsKey")]
    pub r#achievements_key: Option<usize>,
}
