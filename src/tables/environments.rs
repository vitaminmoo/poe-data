#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct Environments {
    pub r#id: String,
    pub r#base_env_file: String,
    pub r#corrupted_env_files: Vec<String>,
    pub r#environment_transitions_key: Option<EnvironmentTransitionsRow>,
    pub r#preload_group: Option<PreloadGroupsRow>,
}

#[allow(non_upper_case_globals)]
pub static TABLE_Environments: LazyLock<Vec<Environments>> = LazyLock::new(|| {
    RAW_TABLE_Environments
        .iter()
        .map(|x| Environments {
            r#id: x.r#id.clone(),
            r#base_env_file: x.r#base_env_file.clone(),
            r#corrupted_env_files: x.r#corrupted_env_files.clone(),
            r#environment_transitions_key: x
                .r#environment_transitions_key
                .map(EnvironmentTransitionsRow),
            r#preload_group: x.r#preload_group.map(PreloadGroupsRow),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct EnvironmentsRow(pub usize);

impl Deref for EnvironmentsRow {
    type Target = Environments;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_Environments[self.0]
    }
}

impl EnvironmentsRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static Environments {
        &TABLE_Environments[self.0]
    }
    pub fn get(&self) -> &'static Environments {
        &TABLE_Environments[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Environments.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static Environments)> {
        TABLE_Environments
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_Environments: LazyLock<Vec<EnvironmentsRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/Environments.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct EnvironmentsRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "Base_ENVFile")]
    pub r#base_env_file: String,
    #[serde(rename = "Corrupted_ENVFiles")]
    pub r#corrupted_env_files: Vec<String>,
    #[serde(rename = "EnvironmentTransitionsKey")]
    pub r#environment_transitions_key: Option<usize>,
    #[serde(rename = "PreloadGroup")]
    pub r#preload_group: Option<usize>,
}
