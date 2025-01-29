#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct SoundEffects {
    pub r#id: String,
    pub r#sound_file: String,
    pub r#sound_file_2_d: String,
}

#[allow(non_upper_case_globals)]
pub static TABLE_SoundEffects: LazyLock<Vec<SoundEffects>> = LazyLock::new(|| {
    RAW_TABLE_SoundEffects
        .iter()
        .map(|x| SoundEffects {
            r#id: x.r#id.clone(),
            r#sound_file: x.r#sound_file.clone(),
            r#sound_file_2_d: x.r#sound_file_2_d.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct SoundEffectsRow(pub usize);

impl Deref for SoundEffectsRow {
    type Target = SoundEffects;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_SoundEffects[self.0]
    }
}

impl SoundEffectsRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static SoundEffects {
        &TABLE_SoundEffects[self.0]
    }
    pub fn get(&self) -> &'static SoundEffects {
        &TABLE_SoundEffects[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_SoundEffects.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static SoundEffects)> {
        TABLE_SoundEffects
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_SoundEffects: LazyLock<Vec<SoundEffectsRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/SoundEffects.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct SoundEffectsRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "SoundFile")]
    pub r#sound_file: String,
    #[serde(rename = "SoundFile_2D")]
    pub r#sound_file_2_d: String,
}
