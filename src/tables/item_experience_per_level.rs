#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ItemExperiencePerLevel: LazyLock<Vec<ItemExperiencePerLevelRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/itemexperienceperlevel.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| ItemExperiencePerLevelRow {
                r#item_experience_type: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(0..0 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    ItemExperienceTypesRef::new(value as usize)
                },
                r#item_current_level: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(16..16 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#experience: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(20..20 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#level: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(24..24 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct ItemExperiencePerLevelRow {
    pub r#item_experience_type: ItemExperienceTypesRef,
    pub r#item_current_level: i32,
    pub r#experience: i32,
    pub r#level: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ItemExperiencePerLevelRef(pub usize);

impl Deref for ItemExperiencePerLevelRef {
    type Target = ItemExperiencePerLevelRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ItemExperiencePerLevel[self.0]
    }
}

impl ItemExperiencePerLevelRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ItemExperiencePerLevelRow {
        &TABLE_ItemExperiencePerLevel[self.0]
    }
    pub fn get(&self) -> &'static ItemExperiencePerLevelRow {
        &TABLE_ItemExperiencePerLevel[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ItemExperiencePerLevel
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ItemExperiencePerLevelRow)> {
        TABLE_ItemExperiencePerLevel
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
        for row in TABLE_ItemExperiencePerLevel.iter() {
            black_box(row);
        }
    }
}
