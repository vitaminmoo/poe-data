#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct MiscAnimated {
    pub r#id: String,
    pub r#ao_file: String,
    pub r#preload_groups_keys: Vec<PreloadGroupsRow>,
    pub r#hash32: i32,
}

#[allow(non_upper_case_globals)]
pub static TABLE_MiscAnimated: LazyLock<Vec<MiscAnimated>> = LazyLock::new(|| {
    RAW_TABLE_MiscAnimated
        .iter()
        .map(|x| MiscAnimated {
            r#id: x.r#id.clone(),
            r#ao_file: x.r#ao_file.clone(),
            r#preload_groups_keys: x
                .r#preload_groups_keys
                .iter()
                .copied()
                .map(PreloadGroupsRow)
                .collect(),
            r#hash32: x.r#hash32.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MiscAnimatedRow(pub usize);

impl Deref for MiscAnimatedRow {
    type Target = MiscAnimated;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_MiscAnimated[self.0]
    }
}

impl MiscAnimatedRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MiscAnimated {
        &TABLE_MiscAnimated[self.0]
    }
    pub fn get(&self) -> &'static MiscAnimated {
        &TABLE_MiscAnimated[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MiscAnimated.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MiscAnimated)> {
        TABLE_MiscAnimated
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_MiscAnimated: LazyLock<Vec<MiscAnimatedRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/MiscAnimated.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct MiscAnimatedRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "AOFile")]
    pub r#ao_file: String,
    #[serde(rename = "PreloadGroupsKeys")]
    pub r#preload_groups_keys: Vec<usize>,
    #[serde(rename = "HASH32")]
    pub r#hash32: i32,
}
