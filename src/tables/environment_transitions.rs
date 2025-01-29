#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct EnvironmentTransitions {
    pub r#id: String,
    pub r#ot_files: Vec<String>,
}

#[allow(non_upper_case_globals)]
pub static TABLE_EnvironmentTransitions: LazyLock<Vec<EnvironmentTransitions>> =
    LazyLock::new(|| {
        RAW_TABLE_EnvironmentTransitions
            .iter()
            .map(|x| EnvironmentTransitions {
                r#id: x.r#id.clone(),
                r#ot_files: x.r#ot_files.clone(),
            })
            .collect()
    });

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct EnvironmentTransitionsRow(pub usize);

impl Deref for EnvironmentTransitionsRow {
    type Target = EnvironmentTransitions;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_EnvironmentTransitions[self.0]
    }
}

impl EnvironmentTransitionsRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static EnvironmentTransitions {
        &TABLE_EnvironmentTransitions[self.0]
    }
    pub fn get(&self) -> &'static EnvironmentTransitions {
        &TABLE_EnvironmentTransitions[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_EnvironmentTransitions
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static EnvironmentTransitions)> {
        TABLE_EnvironmentTransitions
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_EnvironmentTransitions: LazyLock<Vec<EnvironmentTransitionsRaw>> =
    LazyLock::new(|| {
        const DATA: &str = include_str!("../../data/tables/English/EnvironmentTransitions.json");
        serde_json::from_str(DATA).unwrap()
    });

#[derive(Debug, Deserialize, Serialize)]
struct EnvironmentTransitionsRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "OTFiles")]
    pub r#ot_files: Vec<String>,
}
