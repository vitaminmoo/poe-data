#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct BuffVisualOrbs {
    pub r#id: String,
    pub r#buff_visual_orb_types_key: Option<BuffVisualOrbTypesRow>,
    pub r#buff_visual_orb_art_keys: Vec<BuffVisualOrbArtRow>,
    pub r#player_buff_visual_orb_art_keys: Vec<BuffVisualOrbArtRow>,
    pub r#buff_visual_orb_art_keys2: Vec<BuffVisualOrbArtRow>,
}

#[allow(non_upper_case_globals)]
pub static TABLE_BuffVisualOrbs: LazyLock<Vec<BuffVisualOrbs>> = LazyLock::new(|| {
    RAW_TABLE_BuffVisualOrbs
        .iter()
        .map(|x| BuffVisualOrbs {
            r#id: x.r#id.clone(),
            r#buff_visual_orb_types_key: x.r#buff_visual_orb_types_key.map(BuffVisualOrbTypesRow),
            r#buff_visual_orb_art_keys: x
                .r#buff_visual_orb_art_keys
                .iter()
                .copied()
                .map(BuffVisualOrbArtRow)
                .collect(),
            r#player_buff_visual_orb_art_keys: x
                .r#player_buff_visual_orb_art_keys
                .iter()
                .copied()
                .map(BuffVisualOrbArtRow)
                .collect(),
            r#buff_visual_orb_art_keys2: x
                .r#buff_visual_orb_art_keys2
                .iter()
                .copied()
                .map(BuffVisualOrbArtRow)
                .collect(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BuffVisualOrbsRow(pub usize);

impl Deref for BuffVisualOrbsRow {
    type Target = BuffVisualOrbs;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_BuffVisualOrbs[self.0]
    }
}

impl BuffVisualOrbsRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BuffVisualOrbs {
        &TABLE_BuffVisualOrbs[self.0]
    }
    pub fn get(&self) -> &'static BuffVisualOrbs {
        &TABLE_BuffVisualOrbs[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BuffVisualOrbs
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BuffVisualOrbs)> {
        TABLE_BuffVisualOrbs
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_BuffVisualOrbs: LazyLock<Vec<BuffVisualOrbsRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/BuffVisualOrbs.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct BuffVisualOrbsRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "BuffVisualOrbTypesKey")]
    pub r#buff_visual_orb_types_key: Option<usize>,
    #[serde(rename = "BuffVisualOrbArtKeys")]
    pub r#buff_visual_orb_art_keys: Vec<usize>,
    #[serde(rename = "Player_BuffVisualOrbArtKeys")]
    pub r#player_buff_visual_orb_art_keys: Vec<usize>,
    #[serde(rename = "BuffVisualOrbArtKeys2")]
    pub r#buff_visual_orb_art_keys2: Vec<usize>,
}
