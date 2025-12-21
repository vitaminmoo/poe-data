#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_LeagueInfo: LazyLock<Vec<LeagueInfoRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/leagueinfo.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| LeagueInfoRow {
            r#panel_version: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                LeagueInfoPanelVersionsRef::new(value as usize)
            },
            r#panel_id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#panel_image: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(24..24 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#header_image: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(32..32 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#screenshots: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(40..40 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#description: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(56..56 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown64: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(64).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#trailer_video_link: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(65..65 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#background_image: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(73..73 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown81: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(81).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown82: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(82).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#panel_items: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(83..83 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct LeagueInfoRow {
    pub r#panel_version: LeagueInfoPanelVersionsRef,
    pub r#panel_id: String,
    pub r#panel_image: String,
    pub r#header_image: String,
    pub r#screenshots: Vec<String>,
    pub r#description: String,
    pub r#unknown64: bool,
    pub r#trailer_video_link: String,
    pub r#background_image: String,
    pub r#unknown81: bool,
    pub r#unknown82: bool,
    pub r#panel_items: Vec<String>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct LeagueInfoRef(pub usize);

impl Deref for LeagueInfoRef {
    type Target = LeagueInfoRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_LeagueInfo[self.0]
    }
}

impl LeagueInfoRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static LeagueInfoRow {
        &TABLE_LeagueInfo[self.0]
    }
    pub fn get(&self) -> &'static LeagueInfoRow {
        &TABLE_LeagueInfo[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_LeagueInfo.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static LeagueInfoRow)> {
        TABLE_LeagueInfo
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_LeagueInfo.iter() {
            black_box(row);
        }
    }
}
