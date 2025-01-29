#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct AlternateSkillTargetingBehaviours {
    pub r#id: String,
    pub r#client_strings: Option<ClientStringsRow>,
}

#[allow(non_upper_case_globals)]
pub static TABLE_AlternateSkillTargetingBehaviours: LazyLock<
    Vec<AlternateSkillTargetingBehaviours>,
> = LazyLock::new(|| {
    RAW_TABLE_AlternateSkillTargetingBehaviours
        .iter()
        .map(|x| AlternateSkillTargetingBehaviours {
            r#id: x.r#id.clone(),
            r#client_strings: x.r#client_strings.map(ClientStringsRow),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct AlternateSkillTargetingBehavioursRow(pub usize);

impl Deref for AlternateSkillTargetingBehavioursRow {
    type Target = AlternateSkillTargetingBehaviours;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_AlternateSkillTargetingBehaviours[self.0]
    }
}

impl AlternateSkillTargetingBehavioursRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static AlternateSkillTargetingBehaviours {
        &TABLE_AlternateSkillTargetingBehaviours[self.0]
    }
    pub fn get(&self) -> &'static AlternateSkillTargetingBehaviours {
        &TABLE_AlternateSkillTargetingBehaviours[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_AlternateSkillTargetingBehaviours
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs(
    ) -> impl Iterator<Item = (Self, &'static AlternateSkillTargetingBehaviours)> {
        TABLE_AlternateSkillTargetingBehaviours
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_AlternateSkillTargetingBehaviours: LazyLock<
    Vec<AlternateSkillTargetingBehavioursRaw>,
> = LazyLock::new(|| {
    const DATA: &str =
        include_str!("../../data/tables/English/AlternateSkillTargetingBehaviours.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct AlternateSkillTargetingBehavioursRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "ClientStrings")]
    pub r#client_strings: Option<usize>,
}
