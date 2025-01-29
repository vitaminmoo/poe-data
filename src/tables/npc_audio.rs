#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct NPCAudio {
    pub r#id: String,
    pub r#volume_percentage: i32,
}

#[allow(non_upper_case_globals)]
pub static TABLE_NPCAudio: LazyLock<Vec<NPCAudio>> = LazyLock::new(|| {
    RAW_TABLE_NPCAudio
        .iter()
        .map(|x| NPCAudio {
            r#id: x.r#id.clone(),
            r#volume_percentage: x.r#volume_percentage.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct NPCAudioRow(pub usize);

impl Deref for NPCAudioRow {
    type Target = NPCAudio;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_NPCAudio[self.0]
    }
}

impl NPCAudioRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static NPCAudio {
        &TABLE_NPCAudio[self.0]
    }
    pub fn get(&self) -> &'static NPCAudio {
        &TABLE_NPCAudio[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_NPCAudio.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static NPCAudio)> {
        TABLE_NPCAudio.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_NPCAudio: LazyLock<Vec<NPCAudioRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/NPCAudio.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct NPCAudioRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "VolumePercentage")]
    pub r#volume_percentage: i32,
}
