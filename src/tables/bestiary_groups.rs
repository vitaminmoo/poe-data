#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_BestiaryGroups: LazyLock<Vec<BestiaryGroupsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/bestiarygroups.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| BestiaryGroupsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#description: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#illustration: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(24..24 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#icon: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(32..32 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#icon_small: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(40..40 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#bestiary_families_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(48..48 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BestiaryFamiliesRef::new(value as usize)
            },
            r#achievement_items_keys: {
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
                values.into_iter().map(|value| AchievementItemsRef::new(value as usize)).collect()
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct BestiaryGroupsRow {
    pub r#id: String,
    pub r#description: String,
    pub r#illustration: String,
    pub r#name: String,
    pub r#icon: String,
    pub r#icon_small: String,
    pub r#bestiary_families_key: BestiaryFamiliesRef,
    pub r#achievement_items_keys: Vec<AchievementItemsRef>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BestiaryGroupsRef(pub usize);

impl Deref for BestiaryGroupsRef {
    type Target = BestiaryGroupsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_BestiaryGroups[self.0]
    }
}

impl BestiaryGroupsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BestiaryGroupsRow {
        &TABLE_BestiaryGroups[self.0]
    }
    pub fn get(&self) -> &'static BestiaryGroupsRow {
        &TABLE_BestiaryGroups[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BestiaryGroups.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BestiaryGroupsRow)> {
        TABLE_BestiaryGroups.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_BestiaryGroups.iter() {
            black_box(row);
        }
    }
}
