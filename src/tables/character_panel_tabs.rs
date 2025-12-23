#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_CharacterPanelTabs: LazyLock<Vec<CharacterPanelTabsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/characterpaneltabs.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| CharacterPanelTabsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown8: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#text: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(12..12 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown20: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct CharacterPanelTabsRow {
    pub r#id: String,
    pub r#unknown8: i32,
    pub r#text: String,
    pub r#unknown20: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct CharacterPanelTabsRef(pub usize);

impl Deref for CharacterPanelTabsRef {
    type Target = CharacterPanelTabsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_CharacterPanelTabs[self.0]
    }
}

impl CharacterPanelTabsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static CharacterPanelTabsRow {
        &TABLE_CharacterPanelTabs[self.0]
    }
    pub fn get(&self) -> &'static CharacterPanelTabsRow {
        &TABLE_CharacterPanelTabs[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_CharacterPanelTabs.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static CharacterPanelTabsRow)> {
        TABLE_CharacterPanelTabs.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_CharacterPanelTabs.iter() {
            black_box(row);
        }
    }
}
