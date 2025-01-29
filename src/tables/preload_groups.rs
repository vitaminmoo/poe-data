#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct PreloadGroups {
    pub r#id: String,
}

#[allow(non_upper_case_globals)]
pub static TABLE_PreloadGroups: LazyLock<Vec<PreloadGroups>> = LazyLock::new(|| {
    RAW_TABLE_PreloadGroups
        .iter()
        .map(|x| PreloadGroups {
            r#id: x.r#id.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct PreloadGroupsRow(pub usize);

impl Deref for PreloadGroupsRow {
    type Target = PreloadGroups;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_PreloadGroups[self.0]
    }
}

impl PreloadGroupsRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static PreloadGroups {
        &TABLE_PreloadGroups[self.0]
    }
    pub fn get(&self) -> &'static PreloadGroups {
        &TABLE_PreloadGroups[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_PreloadGroups.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static PreloadGroups)> {
        TABLE_PreloadGroups
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_PreloadGroups: LazyLock<Vec<PreloadGroupsRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/PreloadGroups.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct PreloadGroupsRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
}
