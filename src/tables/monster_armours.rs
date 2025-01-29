#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct MonsterArmours {
    pub r#id: String,
    pub r#art_string_sm_file: String,
}

#[allow(non_upper_case_globals)]
pub static TABLE_MonsterArmours: LazyLock<Vec<MonsterArmours>> = LazyLock::new(|| {
    RAW_TABLE_MonsterArmours
        .iter()
        .map(|x| MonsterArmours {
            r#id: x.r#id.clone(),
            r#art_string_sm_file: x.r#art_string_sm_file.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MonsterArmoursRow(pub usize);

impl Deref for MonsterArmoursRow {
    type Target = MonsterArmours;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_MonsterArmours[self.0]
    }
}

impl MonsterArmoursRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MonsterArmours {
        &TABLE_MonsterArmours[self.0]
    }
    pub fn get(&self) -> &'static MonsterArmours {
        &TABLE_MonsterArmours[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MonsterArmours
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MonsterArmours)> {
        TABLE_MonsterArmours
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_MonsterArmours: LazyLock<Vec<MonsterArmoursRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/MonsterArmours.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct MonsterArmoursRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "ArtString_SMFile")]
    pub r#art_string_sm_file: String,
}
