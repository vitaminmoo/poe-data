#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct MonsterSegments {
    pub r#id: String,
    pub r#shapes: String,
}

#[allow(non_upper_case_globals)]
pub static TABLE_MonsterSegments: LazyLock<Vec<MonsterSegments>> = LazyLock::new(|| {
    RAW_TABLE_MonsterSegments
        .iter()
        .map(|x| MonsterSegments {
            r#id: x.r#id.clone(),
            r#shapes: x.r#shapes.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MonsterSegmentsRow(pub usize);

impl Deref for MonsterSegmentsRow {
    type Target = MonsterSegments;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_MonsterSegments[self.0]
    }
}

impl MonsterSegmentsRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MonsterSegments {
        &TABLE_MonsterSegments[self.0]
    }
    pub fn get(&self) -> &'static MonsterSegments {
        &TABLE_MonsterSegments[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MonsterSegments
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MonsterSegments)> {
        TABLE_MonsterSegments
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_MonsterSegments: LazyLock<Vec<MonsterSegmentsRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/MonsterSegments.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct MonsterSegmentsRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "Shapes")]
    pub r#shapes: String,
}
