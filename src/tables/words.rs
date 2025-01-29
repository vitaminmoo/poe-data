#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct Words {
    pub r#wordlist: MaybeVariant<Wordlists>,
    pub r#text: String,
    pub r#spawn_weight_tags: Vec<TagsRow>,
    pub r#spawn_weight_values: Vec<i32>,
    pub r#text2: String,
    pub r#inflection: String,
}

#[allow(non_upper_case_globals)]
pub static TABLE_Words: LazyLock<Vec<Words>> = LazyLock::new(|| {
    RAW_TABLE_Words
        .iter()
        .map(|x| Words {
            r#wordlist: Wordlists::from_repr(x.r#wordlist).map_or(
                MaybeVariant::NotVariant(x.r#wordlist),
                MaybeVariant::Variant,
            ),
            r#text: x.r#text.clone(),
            r#spawn_weight_tags: x.r#spawn_weight_tags.iter().copied().map(TagsRow).collect(),
            r#spawn_weight_values: x.r#spawn_weight_values.clone(),
            r#text2: x.r#text2.clone(),
            r#inflection: x.r#inflection.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct WordsRow(pub usize);

impl Deref for WordsRow {
    type Target = Words;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_Words[self.0]
    }
}

impl WordsRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static Words {
        &TABLE_Words[self.0]
    }
    pub fn get(&self) -> &'static Words {
        &TABLE_Words[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Words.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static Words)> {
        TABLE_Words.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_Words: LazyLock<Vec<WordsRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/Words.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct WordsRaw {
    #[serde(rename = "Wordlist")]
    pub r#wordlist: usize,
    #[serde(rename = "Text")]
    pub r#text: String,
    #[serde(rename = "SpawnWeight_Tags")]
    pub r#spawn_weight_tags: Vec<usize>,
    #[serde(rename = "SpawnWeight_Values")]
    pub r#spawn_weight_values: Vec<i32>,
    #[serde(rename = "Text2")]
    pub r#text2: String,
    #[serde(rename = "Inflection")]
    pub r#inflection: String,
}
