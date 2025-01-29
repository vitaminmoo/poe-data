#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct MicrotransactionSkillGemEffectSlotTypes {
    pub r#id: String,
    pub r#type: String,
}

#[allow(non_upper_case_globals)]
pub static TABLE_MicrotransactionSkillGemEffectSlotTypes: LazyLock<
    Vec<MicrotransactionSkillGemEffectSlotTypes>,
> = LazyLock::new(|| {
    RAW_TABLE_MicrotransactionSkillGemEffectSlotTypes
        .iter()
        .map(|x| MicrotransactionSkillGemEffectSlotTypes {
            r#id: x.r#id.clone(),
            r#type: x.r#type.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MicrotransactionSkillGemEffectSlotTypesRow(pub usize);

impl Deref for MicrotransactionSkillGemEffectSlotTypesRow {
    type Target = MicrotransactionSkillGemEffectSlotTypes;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_MicrotransactionSkillGemEffectSlotTypes[self.0]
    }
}

impl MicrotransactionSkillGemEffectSlotTypesRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MicrotransactionSkillGemEffectSlotTypes {
        &TABLE_MicrotransactionSkillGemEffectSlotTypes[self.0]
    }
    pub fn get(&self) -> &'static MicrotransactionSkillGemEffectSlotTypes {
        &TABLE_MicrotransactionSkillGemEffectSlotTypes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MicrotransactionSkillGemEffectSlotTypes
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs(
    ) -> impl Iterator<Item = (Self, &'static MicrotransactionSkillGemEffectSlotTypes)> {
        TABLE_MicrotransactionSkillGemEffectSlotTypes
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_MicrotransactionSkillGemEffectSlotTypes: LazyLock<
    Vec<MicrotransactionSkillGemEffectSlotTypesRaw>,
> = LazyLock::new(|| {
    const DATA: &str =
        include_str!("../../data/tables/English/MicrotransactionSkillGemEffectSlotTypes.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct MicrotransactionSkillGemEffectSlotTypesRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "Type")]
    pub r#type: String,
}
