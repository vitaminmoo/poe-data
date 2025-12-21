#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_BlightCraftingItems: LazyLock<Vec<BlightCraftingItemsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/blightcraftingitems.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| BlightCraftingItemsRow {
            r#base_item_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#tier: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#achievements: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(20..20 + 16).unwrap();
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
                    .map(|value| AchievementItemsRef::new(value as usize))
                    .collect()
            },
            r#use_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(36..36 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#name_short: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(40..40 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#enchanted_mod: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(48..48 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ModsRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct BlightCraftingItemsRow {
    pub r#base_item_type: BaseItemTypesRef,
    pub r#tier: i32,
    pub r#achievements: Vec<AchievementItemsRef>,
    pub r#use_type: i32,
    pub r#name_short: String,
    pub r#enchanted_mod: ModsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BlightCraftingItemsRef(pub usize);

impl Deref for BlightCraftingItemsRef {
    type Target = BlightCraftingItemsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_BlightCraftingItems[self.0]
    }
}

impl BlightCraftingItemsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BlightCraftingItemsRow {
        &TABLE_BlightCraftingItems[self.0]
    }
    pub fn get(&self) -> &'static BlightCraftingItemsRow {
        &TABLE_BlightCraftingItems[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BlightCraftingItems
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BlightCraftingItemsRow)> {
        TABLE_BlightCraftingItems
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
        for row in TABLE_BlightCraftingItems.iter() {
            black_box(row);
        }
    }
}
