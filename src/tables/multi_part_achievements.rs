#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct MultiPartAchievements {
    pub r#id: String,
    pub r#achievement_items_key: Option<AchievementItemsRow>,
    pub r#threshold: i32,
}

#[allow(non_upper_case_globals)]
pub static TABLE_MultiPartAchievements: LazyLock<Vec<MultiPartAchievements>> =
    LazyLock::new(|| {
        RAW_TABLE_MultiPartAchievements
            .iter()
            .map(|x| MultiPartAchievements {
                r#id: x.r#id.clone(),
                r#achievement_items_key: x.r#achievement_items_key.map(AchievementItemsRow),
                r#threshold: x.r#threshold.clone(),
            })
            .collect()
    });

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MultiPartAchievementsRow(pub usize);

impl Deref for MultiPartAchievementsRow {
    type Target = MultiPartAchievements;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_MultiPartAchievements[self.0]
    }
}

impl MultiPartAchievementsRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MultiPartAchievements {
        &TABLE_MultiPartAchievements[self.0]
    }
    pub fn get(&self) -> &'static MultiPartAchievements {
        &TABLE_MultiPartAchievements[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MultiPartAchievements
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MultiPartAchievements)> {
        TABLE_MultiPartAchievements
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_MultiPartAchievements: LazyLock<Vec<MultiPartAchievementsRaw>> =
    LazyLock::new(|| {
        const DATA: &str = include_str!("../../data/tables/English/MultiPartAchievements.json");
        serde_json::from_str(DATA).unwrap()
    });

#[derive(Debug, Deserialize, Serialize)]
struct MultiPartAchievementsRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "AchievementItemsKey")]
    pub r#achievement_items_key: Option<usize>,
    #[serde(rename = "Threshold")]
    pub r#threshold: i32,
}
