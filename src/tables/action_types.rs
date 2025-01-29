#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct ActionTypes {
    pub r#id: String,
}

#[allow(non_upper_case_globals)]
pub static TABLE_ActionTypes: LazyLock<Vec<ActionTypes>> = LazyLock::new(|| {
    RAW_TABLE_ActionTypes
        .iter()
        .map(|x| ActionTypes {
            r#id: x.r#id.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ActionTypesRow(pub usize);

impl Deref for ActionTypesRow {
    type Target = ActionTypes;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_ActionTypes[self.0]
    }
}

impl ActionTypesRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ActionTypes {
        &TABLE_ActionTypes[self.0]
    }
    pub fn get(&self) -> &'static ActionTypes {
        &TABLE_ActionTypes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ActionTypes.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ActionTypes)> {
        TABLE_ActionTypes
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_ActionTypes: LazyLock<Vec<ActionTypesRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/ActionTypes.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct ActionTypesRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
}
