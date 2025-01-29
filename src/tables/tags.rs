#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct Tags {
    pub r#id: String,
    pub r#display_string: String,
    pub r#name: String,
}

#[allow(non_upper_case_globals)]
pub static TABLE_Tags: LazyLock<Vec<Tags>> = LazyLock::new(|| {
    RAW_TABLE_Tags
        .iter()
        .map(|x| Tags {
            r#id: x.r#id.clone(),
            r#display_string: x.r#display_string.clone(),
            r#name: x.r#name.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct TagsRow(pub usize);

impl Deref for TagsRow {
    type Target = Tags;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_Tags[self.0]
    }
}

impl TagsRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static Tags {
        &TABLE_Tags[self.0]
    }
    pub fn get(&self) -> &'static Tags {
        &TABLE_Tags[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Tags.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static Tags)> {
        TABLE_Tags.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_Tags: LazyLock<Vec<TagsRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/Tags.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct TagsRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "DisplayString")]
    pub r#display_string: String,
    #[serde(rename = "Name")]
    pub r#name: String,
}
