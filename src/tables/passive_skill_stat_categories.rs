#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct PassiveSkillStatCategories {
    pub r#id: String,
    pub r#name: String,
}

#[allow(non_upper_case_globals)]
pub static TABLE_PassiveSkillStatCategories: LazyLock<Vec<PassiveSkillStatCategories>> =
    LazyLock::new(|| {
        RAW_TABLE_PassiveSkillStatCategories
            .iter()
            .map(|x| PassiveSkillStatCategories {
                r#id: x.r#id.clone(),
                r#name: x.r#name.clone(),
            })
            .collect()
    });

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct PassiveSkillStatCategoriesRow(pub usize);

impl Deref for PassiveSkillStatCategoriesRow {
    type Target = PassiveSkillStatCategories;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_PassiveSkillStatCategories[self.0]
    }
}

impl PassiveSkillStatCategoriesRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static PassiveSkillStatCategories {
        &TABLE_PassiveSkillStatCategories[self.0]
    }
    pub fn get(&self) -> &'static PassiveSkillStatCategories {
        &TABLE_PassiveSkillStatCategories[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_PassiveSkillStatCategories
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static PassiveSkillStatCategories)> {
        TABLE_PassiveSkillStatCategories
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_PassiveSkillStatCategories: LazyLock<Vec<PassiveSkillStatCategoriesRaw>> =
    LazyLock::new(|| {
        const DATA: &str =
            include_str!("../../data/tables/English/PassiveSkillStatCategories.json");
        serde_json::from_str(DATA).unwrap()
    });

#[derive(Debug, Deserialize, Serialize)]
struct PassiveSkillStatCategoriesRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "Name")]
    pub r#name: String,
}
