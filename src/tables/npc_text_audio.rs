#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct NPCTextAudio {
    pub r#id: String,
    pub r#characters: Vec<CharactersRow>,
    pub r#text: String,
    pub r#mono_audio_file: String,
    pub r#stereo_audio_file: String,
    pub r#has_stereo: bool,
    pub r#video: String,
    pub r#np_cs: Vec<NPCsRow>,
}

#[allow(non_upper_case_globals)]
pub static TABLE_NPCTextAudio: LazyLock<Vec<NPCTextAudio>> = LazyLock::new(|| {
    RAW_TABLE_NPCTextAudio
        .iter()
        .map(|x| NPCTextAudio {
            r#id: x.r#id.clone(),
            r#characters: x.r#characters.iter().copied().map(CharactersRow).collect(),
            r#text: x.r#text.clone(),
            r#mono_audio_file: x.r#mono_audio_file.clone(),
            r#stereo_audio_file: x.r#stereo_audio_file.clone(),
            r#has_stereo: x.r#has_stereo.clone(),
            r#video: x.r#video.clone(),
            r#np_cs: x.r#np_cs.iter().copied().map(NPCsRow).collect(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct NPCTextAudioRow(pub usize);

impl Deref for NPCTextAudioRow {
    type Target = NPCTextAudio;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_NPCTextAudio[self.0]
    }
}

impl NPCTextAudioRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static NPCTextAudio {
        &TABLE_NPCTextAudio[self.0]
    }
    pub fn get(&self) -> &'static NPCTextAudio {
        &TABLE_NPCTextAudio[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_NPCTextAudio.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static NPCTextAudio)> {
        TABLE_NPCTextAudio
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_NPCTextAudio: LazyLock<Vec<NPCTextAudioRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/NPCTextAudio.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct NPCTextAudioRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "Characters")]
    pub r#characters: Vec<usize>,
    #[serde(rename = "Text")]
    pub r#text: String,
    #[serde(rename = "Mono_AudioFile")]
    pub r#mono_audio_file: String,
    #[serde(rename = "Stereo_AudioFile")]
    pub r#stereo_audio_file: String,
    #[serde(rename = "HasStereo")]
    pub r#has_stereo: bool,
    #[serde(rename = "Video")]
    pub r#video: String,
    #[serde(rename = "NPCs")]
    pub r#np_cs: Vec<usize>,
}
