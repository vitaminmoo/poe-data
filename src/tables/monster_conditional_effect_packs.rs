#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct MonsterConditionalEffectPacks {
    pub r#id: String,
    pub r#misc_effect_pack1: Vec<MiscEffectPacksRow>,
    pub r#misc_effect_pack2: Vec<MiscEffectPacksRow>,
    pub r#misc_effect_pack3: Vec<MiscEffectPacksRow>,
    pub r#misc_effect_pack4: Vec<MiscEffectPacksRow>,
}

#[allow(non_upper_case_globals)]
pub static TABLE_MonsterConditionalEffectPacks: LazyLock<Vec<MonsterConditionalEffectPacks>> =
    LazyLock::new(|| {
        RAW_TABLE_MonsterConditionalEffectPacks
            .iter()
            .map(|x| MonsterConditionalEffectPacks {
                r#id: x.r#id.clone(),
                r#misc_effect_pack1: x
                    .r#misc_effect_pack1
                    .iter()
                    .copied()
                    .map(MiscEffectPacksRow)
                    .collect(),
                r#misc_effect_pack2: x
                    .r#misc_effect_pack2
                    .iter()
                    .copied()
                    .map(MiscEffectPacksRow)
                    .collect(),
                r#misc_effect_pack3: x
                    .r#misc_effect_pack3
                    .iter()
                    .copied()
                    .map(MiscEffectPacksRow)
                    .collect(),
                r#misc_effect_pack4: x
                    .r#misc_effect_pack4
                    .iter()
                    .copied()
                    .map(MiscEffectPacksRow)
                    .collect(),
            })
            .collect()
    });

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MonsterConditionalEffectPacksRow(pub usize);

impl Deref for MonsterConditionalEffectPacksRow {
    type Target = MonsterConditionalEffectPacks;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_MonsterConditionalEffectPacks[self.0]
    }
}

impl MonsterConditionalEffectPacksRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MonsterConditionalEffectPacks {
        &TABLE_MonsterConditionalEffectPacks[self.0]
    }
    pub fn get(&self) -> &'static MonsterConditionalEffectPacks {
        &TABLE_MonsterConditionalEffectPacks[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MonsterConditionalEffectPacks
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MonsterConditionalEffectPacks)>
    {
        TABLE_MonsterConditionalEffectPacks
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_MonsterConditionalEffectPacks: LazyLock<Vec<MonsterConditionalEffectPacksRaw>> =
    LazyLock::new(|| {
        const DATA: &str =
            include_str!("../../data/tables/English/MonsterConditionalEffectPacks.json");
        serde_json::from_str(DATA).unwrap()
    });

#[derive(Debug, Deserialize, Serialize)]
struct MonsterConditionalEffectPacksRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "MiscEffectPack1")]
    pub r#misc_effect_pack1: Vec<usize>,
    #[serde(rename = "MiscEffectPack2")]
    pub r#misc_effect_pack2: Vec<usize>,
    #[serde(rename = "MiscEffectPack3")]
    pub r#misc_effect_pack3: Vec<usize>,
    #[serde(rename = "MiscEffectPack4")]
    pub r#misc_effect_pack4: Vec<usize>,
}
