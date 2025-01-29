#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct Animation {
    pub r#id: String,
    pub r#mainhand_animation_key: Option<AnimationRow>,
    pub r#offhand_animation_key: Option<AnimationRow>,
}

#[allow(non_upper_case_globals)]
pub static TABLE_Animation: LazyLock<Vec<Animation>> = LazyLock::new(|| {
    RAW_TABLE_Animation
        .iter()
        .map(|x| Animation {
            r#id: x.r#id.clone(),
            r#mainhand_animation_key: x.r#mainhand_animation_key.map(AnimationRow),
            r#offhand_animation_key: x.r#offhand_animation_key.map(AnimationRow),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct AnimationRow(pub usize);

impl Deref for AnimationRow {
    type Target = Animation;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_Animation[self.0]
    }
}

impl AnimationRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static Animation {
        &TABLE_Animation[self.0]
    }
    pub fn get(&self) -> &'static Animation {
        &TABLE_Animation[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Animation.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static Animation)> {
        TABLE_Animation
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_Animation: LazyLock<Vec<AnimationRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/Animation.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct AnimationRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "Mainhand_AnimationKey")]
    pub r#mainhand_animation_key: Option<usize>,
    #[serde(rename = "Offhand_AnimationKey")]
    pub r#offhand_animation_key: Option<usize>,
}
