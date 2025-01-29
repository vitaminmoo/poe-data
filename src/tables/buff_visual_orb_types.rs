#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct BuffVisualOrbTypes {
    pub r#id: String,
}

#[allow(non_upper_case_globals)]
pub static TABLE_BuffVisualOrbTypes: LazyLock<Vec<BuffVisualOrbTypes>> = LazyLock::new(|| {
    RAW_TABLE_BuffVisualOrbTypes
        .iter()
        .map(|x| BuffVisualOrbTypes {
            r#id: x.r#id.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BuffVisualOrbTypesRow(pub usize);

impl Deref for BuffVisualOrbTypesRow {
    type Target = BuffVisualOrbTypes;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_BuffVisualOrbTypes[self.0]
    }
}

impl BuffVisualOrbTypesRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BuffVisualOrbTypes {
        &TABLE_BuffVisualOrbTypes[self.0]
    }
    pub fn get(&self) -> &'static BuffVisualOrbTypes {
        &TABLE_BuffVisualOrbTypes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BuffVisualOrbTypes
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BuffVisualOrbTypes)> {
        TABLE_BuffVisualOrbTypes
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_BuffVisualOrbTypes: LazyLock<Vec<BuffVisualOrbTypesRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/BuffVisualOrbTypes.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct BuffVisualOrbTypesRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
}
