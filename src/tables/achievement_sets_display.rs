#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct AchievementSetsDisplay {
    pub r#id: i32,
    pub r#title: String,
}

#[allow(non_upper_case_globals)]
pub static TABLE_AchievementSetsDisplay: LazyLock<Vec<AchievementSetsDisplay>> =
    LazyLock::new(|| {
        RAW_TABLE_AchievementSetsDisplay
            .iter()
            .map(|x| AchievementSetsDisplay {
                r#id: x.r#id.clone(),
                r#title: x.r#title.clone(),
            })
            .collect()
    });

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct AchievementSetsDisplayRow(pub usize);

impl Deref for AchievementSetsDisplayRow {
    type Target = AchievementSetsDisplay;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_AchievementSetsDisplay[self.0]
    }
}

impl AchievementSetsDisplayRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static AchievementSetsDisplay {
        &TABLE_AchievementSetsDisplay[self.0]
    }
    pub fn get(&self) -> &'static AchievementSetsDisplay {
        &TABLE_AchievementSetsDisplay[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_AchievementSetsDisplay
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static AchievementSetsDisplay)> {
        TABLE_AchievementSetsDisplay
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_AchievementSetsDisplay: LazyLock<Vec<AchievementSetsDisplayRaw>> =
    LazyLock::new(|| {
        const DATA: &str = include_str!("../../data/tables/English/AchievementSetsDisplay.json");
        serde_json::from_str(DATA).unwrap()
    });

#[derive(Debug, Deserialize, Serialize)]
struct AchievementSetsDisplayRaw {
    #[serde(rename = "Id")]
    pub r#id: i32,
    #[serde(rename = "Title")]
    pub r#title: String,
}
