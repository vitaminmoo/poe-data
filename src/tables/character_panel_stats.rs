#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_CharacterPanelStats: LazyLock<Vec<CharacterPanelStatsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/characterpanelstats.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| CharacterPanelStatsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#text: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#stats_keys1: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values
                    .into_iter()
                    .map(|value| StatsRef::new(value as usize))
                    .collect()
            },
            r#character_panel_description_modes_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                CharacterPanelDescriptionModesRef::new(value as usize)
            },
            r#stats_keys2: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(48..48 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values
                    .into_iter()
                    .map(|value| StatsRef::new(value as usize))
                    .collect()
            },
            r#stats_keys3: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(64..64 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values
                    .into_iter()
                    .map(|value| StatsRef::new(value as usize))
                    .collect()
            },
            r#character_panel_tabs_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(80..80 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                CharacterPanelTabsRef::new(value as usize)
            },
            r#unknown96: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(96).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown97: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(97..97 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 4)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i32_le())
                    .collect::<Vec<i32>>();
                values
            },
            r#unknown113: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(113..113 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct CharacterPanelStatsRow {
    pub r#id: String,
    pub r#text: String,
    pub r#stats_keys1: Vec<StatsRef>,
    pub r#character_panel_description_modes_key: CharacterPanelDescriptionModesRef,
    pub r#stats_keys2: Vec<StatsRef>,
    pub r#stats_keys3: Vec<StatsRef>,
    pub r#character_panel_tabs_key: CharacterPanelTabsRef,
    pub r#unknown96: bool,
    pub r#unknown97: Vec<i32>,
    pub r#unknown113: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct CharacterPanelStatsRef(pub usize);

impl Deref for CharacterPanelStatsRef {
    type Target = CharacterPanelStatsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_CharacterPanelStats[self.0]
    }
}

impl CharacterPanelStatsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static CharacterPanelStatsRow {
        &TABLE_CharacterPanelStats[self.0]
    }
    pub fn get(&self) -> &'static CharacterPanelStatsRow {
        &TABLE_CharacterPanelStats[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_CharacterPanelStats
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static CharacterPanelStatsRow)> {
        TABLE_CharacterPanelStats
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
        for row in TABLE_CharacterPanelStats.iter() {
            black_box(row);
        }
    }
}
