#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct Achievements {
    pub r#id: String,
    pub r#description: String,
    // column_ref set_id
    pub r#objective: String,
    pub r#hash16: i16,
    pub r#hide_achievement_items: bool,
    pub r#min_completed_items: i32,
    pub r#two_column_layout: bool,
    pub r#show_item_completions_as_one: bool,
    pub r#softcore_only: bool,
    pub r#hardcore_only: bool,
}

#[allow(non_upper_case_globals)]
pub static TABLE_Achievements: LazyLock<Vec<Achievements>> = LazyLock::new(|| {
    RAW_TABLE_Achievements
        .iter()
        .map(|x| {
            Achievements {
                r#id: x.r#id.clone(),
                r#description: x.r#description.clone(),
                // column_ref set_id
                r#objective: x.r#objective.clone(),
                r#hash16: x.r#hash16.clone(),
                r#hide_achievement_items: x.r#hide_achievement_items.clone(),
                r#min_completed_items: x.r#min_completed_items.clone(),
                r#two_column_layout: x.r#two_column_layout.clone(),
                r#show_item_completions_as_one: x.r#show_item_completions_as_one.clone(),
                r#softcore_only: x.r#softcore_only.clone(),
                r#hardcore_only: x.r#hardcore_only.clone(),
            }
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct AchievementsRow(pub usize);

impl Deref for AchievementsRow {
    type Target = Achievements;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_Achievements[self.0]
    }
}

impl AchievementsRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static Achievements {
        &TABLE_Achievements[self.0]
    }
    pub fn get(&self) -> &'static Achievements {
        &TABLE_Achievements[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Achievements.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static Achievements)> {
        TABLE_Achievements
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_Achievements: LazyLock<Vec<AchievementsRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/Achievements.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct AchievementsRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "Description")]
    pub r#description: String,
    #[serde(rename = "SetId")]
    pub r#set_id: i32,
    #[serde(rename = "Objective")]
    pub r#objective: String,
    #[serde(rename = "HASH16")]
    pub r#hash16: i16,
    #[serde(rename = "HideAchievementItems")]
    pub r#hide_achievement_items: bool,
    #[serde(rename = "MinCompletedItems")]
    pub r#min_completed_items: i32,
    #[serde(rename = "TwoColumnLayout")]
    pub r#two_column_layout: bool,
    #[serde(rename = "ShowItemCompletionsAsOne")]
    pub r#show_item_completions_as_one: bool,
    #[serde(rename = "SoftcoreOnly")]
    pub r#softcore_only: bool,
    #[serde(rename = "HardcoreOnly")]
    pub r#hardcore_only: bool,
}
