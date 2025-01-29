#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct ActiveSkillType {
    pub r#id: String,
    pub r#flag_stat: Option<StatsRow>,
}

#[allow(non_upper_case_globals)]
pub static TABLE_ActiveSkillType: LazyLock<Vec<ActiveSkillType>> = LazyLock::new(|| {
    RAW_TABLE_ActiveSkillType
        .iter()
        .map(|x| ActiveSkillType {
            r#id: x.r#id.clone(),
            r#flag_stat: x.r#flag_stat.map(StatsRow),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ActiveSkillTypeRow(pub usize);

impl Deref for ActiveSkillTypeRow {
    type Target = ActiveSkillType;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_ActiveSkillType[self.0]
    }
}

impl ActiveSkillTypeRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ActiveSkillType {
        &TABLE_ActiveSkillType[self.0]
    }
    pub fn get(&self) -> &'static ActiveSkillType {
        &TABLE_ActiveSkillType[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ActiveSkillType
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ActiveSkillType)> {
        TABLE_ActiveSkillType
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_ActiveSkillType: LazyLock<Vec<ActiveSkillTypeRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/ActiveSkillType.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct ActiveSkillTypeRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "FlagStat")]
    pub r#flag_stat: Option<usize>,
}
