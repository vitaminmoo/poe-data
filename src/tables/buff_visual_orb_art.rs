#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct BuffVisualOrbArt {
    pub r#id: String,
    pub r#misc_animated: Option<MiscAnimatedRow>,
}

#[allow(non_upper_case_globals)]
pub static TABLE_BuffVisualOrbArt: LazyLock<Vec<BuffVisualOrbArt>> = LazyLock::new(|| {
    RAW_TABLE_BuffVisualOrbArt
        .iter()
        .map(|x| BuffVisualOrbArt {
            r#id: x.r#id.clone(),
            r#misc_animated: x.r#misc_animated.map(MiscAnimatedRow),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BuffVisualOrbArtRow(pub usize);

impl Deref for BuffVisualOrbArtRow {
    type Target = BuffVisualOrbArt;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_BuffVisualOrbArt[self.0]
    }
}

impl BuffVisualOrbArtRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BuffVisualOrbArt {
        &TABLE_BuffVisualOrbArt[self.0]
    }
    pub fn get(&self) -> &'static BuffVisualOrbArt {
        &TABLE_BuffVisualOrbArt[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BuffVisualOrbArt
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BuffVisualOrbArt)> {
        TABLE_BuffVisualOrbArt
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_BuffVisualOrbArt: LazyLock<Vec<BuffVisualOrbArtRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/BuffVisualOrbArt.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct BuffVisualOrbArtRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "MiscAnimated")]
    pub r#misc_animated: Option<usize>,
}
