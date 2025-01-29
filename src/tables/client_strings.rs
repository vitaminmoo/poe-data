#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct ClientStrings {
    pub r#id: String,
    pub r#text: String,
    pub r#x_box_text: String,
    pub r#x_box_text2: String,
    pub r#hash32: i32,
    pub r#playstation_text: String,
}

#[allow(non_upper_case_globals)]
pub static TABLE_ClientStrings: LazyLock<Vec<ClientStrings>> = LazyLock::new(|| {
    RAW_TABLE_ClientStrings
        .iter()
        .map(|x| ClientStrings {
            r#id: x.r#id.clone(),
            r#text: x.r#text.clone(),
            r#x_box_text: x.r#x_box_text.clone(),
            r#x_box_text2: x.r#x_box_text2.clone(),
            r#hash32: x.r#hash32.clone(),
            r#playstation_text: x.r#playstation_text.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ClientStringsRow(pub usize);

impl Deref for ClientStringsRow {
    type Target = ClientStrings;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_ClientStrings[self.0]
    }
}

impl ClientStringsRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ClientStrings {
        &TABLE_ClientStrings[self.0]
    }
    pub fn get(&self) -> &'static ClientStrings {
        &TABLE_ClientStrings[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ClientStrings.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ClientStrings)> {
        TABLE_ClientStrings
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_ClientStrings: LazyLock<Vec<ClientStringsRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/ClientStrings.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct ClientStringsRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "Text")]
    pub r#text: String,
    #[serde(rename = "XBoxText")]
    pub r#x_box_text: String,
    #[serde(rename = "XBoxText2")]
    pub r#x_box_text2: String,
    #[serde(rename = "HASH32")]
    pub r#hash32: i32,
    #[serde(rename = "PlaystationText")]
    pub r#playstation_text: String,
}
