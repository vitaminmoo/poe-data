#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct VirtualStatContextFlags {
    pub r#id: String,
}

#[allow(non_upper_case_globals)]
pub static TABLE_VirtualStatContextFlags: LazyLock<Vec<VirtualStatContextFlags>> =
    LazyLock::new(|| {
        RAW_TABLE_VirtualStatContextFlags
            .iter()
            .map(|x| VirtualStatContextFlags {
                r#id: x.r#id.clone(),
            })
            .collect()
    });

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct VirtualStatContextFlagsRow(pub usize);

impl Deref for VirtualStatContextFlagsRow {
    type Target = VirtualStatContextFlags;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_VirtualStatContextFlags[self.0]
    }
}

impl VirtualStatContextFlagsRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static VirtualStatContextFlags {
        &TABLE_VirtualStatContextFlags[self.0]
    }
    pub fn get(&self) -> &'static VirtualStatContextFlags {
        &TABLE_VirtualStatContextFlags[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_VirtualStatContextFlags
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static VirtualStatContextFlags)> {
        TABLE_VirtualStatContextFlags
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_VirtualStatContextFlags: LazyLock<Vec<VirtualStatContextFlagsRaw>> =
    LazyLock::new(|| {
        const DATA: &str = include_str!("../../data/tables/English/VirtualStatContextFlags.json");
        serde_json::from_str(DATA).unwrap()
    });

#[derive(Debug, Deserialize, Serialize)]
struct VirtualStatContextFlagsRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
}
