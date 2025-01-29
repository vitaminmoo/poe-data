#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct Rulesets {
    pub r#id: String,
}

#[allow(non_upper_case_globals)]
pub static TABLE_Rulesets: LazyLock<Vec<Rulesets>> = LazyLock::new(|| {
    RAW_TABLE_Rulesets
        .iter()
        .map(|x| Rulesets {
            r#id: x.r#id.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct RulesetsRow(pub usize);

impl Deref for RulesetsRow {
    type Target = Rulesets;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_Rulesets[self.0]
    }
}

impl RulesetsRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static Rulesets {
        &TABLE_Rulesets[self.0]
    }
    pub fn get(&self) -> &'static Rulesets {
        &TABLE_Rulesets[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Rulesets.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static Rulesets)> {
        TABLE_Rulesets.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_Rulesets: LazyLock<Vec<RulesetsRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/Rulesets.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct RulesetsRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
}
