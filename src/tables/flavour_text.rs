#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct FlavourText {
    pub r#id: String,
    pub r#text: String,
}

#[allow(non_upper_case_globals)]
pub static TABLE_FlavourText: LazyLock<Vec<FlavourText>> = LazyLock::new(|| {
    RAW_TABLE_FlavourText
        .iter()
        .map(|x| FlavourText {
            r#id: x.r#id.clone(),
            r#text: x.r#text.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct FlavourTextRow(pub usize);

impl Deref for FlavourTextRow {
    type Target = FlavourText;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_FlavourText[self.0]
    }
}

impl FlavourTextRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static FlavourText {
        &TABLE_FlavourText[self.0]
    }
    pub fn get(&self) -> &'static FlavourText {
        &TABLE_FlavourText[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_FlavourText.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static FlavourText)> {
        TABLE_FlavourText
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_FlavourText: LazyLock<Vec<FlavourTextRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/FlavourText.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct FlavourTextRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "Text")]
    pub r#text: String,
}
