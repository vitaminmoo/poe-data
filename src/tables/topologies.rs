#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct Topologies {
    pub r#id: String,
    pub r#dgr_file: String,
}

#[allow(non_upper_case_globals)]
pub static TABLE_Topologies: LazyLock<Vec<Topologies>> = LazyLock::new(|| {
    RAW_TABLE_Topologies
        .iter()
        .map(|x| Topologies {
            r#id: x.r#id.clone(),
            r#dgr_file: x.r#dgr_file.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct TopologiesRow(pub usize);

impl Deref for TopologiesRow {
    type Target = Topologies;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_Topologies[self.0]
    }
}

impl TopologiesRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static Topologies {
        &TABLE_Topologies[self.0]
    }
    pub fn get(&self) -> &'static Topologies {
        &TABLE_Topologies[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Topologies.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static Topologies)> {
        TABLE_Topologies
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_Topologies: LazyLock<Vec<TopologiesRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/Topologies.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct TopologiesRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "DGRFile")]
    pub r#dgr_file: String,
}
