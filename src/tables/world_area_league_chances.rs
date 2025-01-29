#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct WorldAreaLeagueChances {
    pub r#id: String,
}

#[allow(non_upper_case_globals)]
pub static TABLE_WorldAreaLeagueChances: LazyLock<Vec<WorldAreaLeagueChances>> =
    LazyLock::new(|| {
        RAW_TABLE_WorldAreaLeagueChances
            .iter()
            .map(|x| WorldAreaLeagueChances {
                r#id: x.r#id.clone(),
            })
            .collect()
    });

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct WorldAreaLeagueChancesRow(pub usize);

impl Deref for WorldAreaLeagueChancesRow {
    type Target = WorldAreaLeagueChances;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_WorldAreaLeagueChances[self.0]
    }
}

impl WorldAreaLeagueChancesRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static WorldAreaLeagueChances {
        &TABLE_WorldAreaLeagueChances[self.0]
    }
    pub fn get(&self) -> &'static WorldAreaLeagueChances {
        &TABLE_WorldAreaLeagueChances[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_WorldAreaLeagueChances
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static WorldAreaLeagueChances)> {
        TABLE_WorldAreaLeagueChances
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_WorldAreaLeagueChances: LazyLock<Vec<WorldAreaLeagueChancesRaw>> =
    LazyLock::new(|| {
        const DATA: &str = include_str!("../../data/tables/English/WorldAreaLeagueChances.json");
        serde_json::from_str(DATA).unwrap()
    });

#[derive(Debug, Deserialize, Serialize)]
struct WorldAreaLeagueChancesRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
}
