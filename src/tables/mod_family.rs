#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct ModFamily {
    pub r#id: String,
}

#[allow(non_upper_case_globals)]
pub static TABLE_ModFamily: LazyLock<Vec<ModFamily>> = LazyLock::new(|| {
    RAW_TABLE_ModFamily
        .iter()
        .map(|x| ModFamily {
            r#id: x.r#id.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ModFamilyRow(pub usize);

impl Deref for ModFamilyRow {
    type Target = ModFamily;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_ModFamily[self.0]
    }
}

impl ModFamilyRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ModFamily {
        &TABLE_ModFamily[self.0]
    }
    pub fn get(&self) -> &'static ModFamily {
        &TABLE_ModFamily[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ModFamily.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ModFamily)> {
        TABLE_ModFamily
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_ModFamily: LazyLock<Vec<ModFamilyRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/ModFamily.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct ModFamilyRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
}
