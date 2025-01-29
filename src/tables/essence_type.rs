#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct EssenceType {
    pub r#id: String,
    pub r#essence_type: i32,
    pub r#is_corrupted_essence: bool,
    pub r#words_key: Option<WordsRow>,
}

#[allow(non_upper_case_globals)]
pub static TABLE_EssenceType: LazyLock<Vec<EssenceType>> = LazyLock::new(|| {
    RAW_TABLE_EssenceType
        .iter()
        .map(|x| EssenceType {
            r#id: x.r#id.clone(),
            r#essence_type: x.r#essence_type.clone(),
            r#is_corrupted_essence: x.r#is_corrupted_essence.clone(),
            r#words_key: x.r#words_key.map(WordsRow),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct EssenceTypeRow(pub usize);

impl Deref for EssenceTypeRow {
    type Target = EssenceType;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_EssenceType[self.0]
    }
}

impl EssenceTypeRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static EssenceType {
        &TABLE_EssenceType[self.0]
    }
    pub fn get(&self) -> &'static EssenceType {
        &TABLE_EssenceType[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_EssenceType.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static EssenceType)> {
        TABLE_EssenceType
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_EssenceType: LazyLock<Vec<EssenceTypeRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/EssenceType.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct EssenceTypeRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "EssenceType")]
    pub r#essence_type: i32,
    #[serde(rename = "IsCorruptedEssence")]
    pub r#is_corrupted_essence: bool,
    #[serde(rename = "WordsKey")]
    pub r#words_key: Option<usize>,
}
