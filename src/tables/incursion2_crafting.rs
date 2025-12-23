#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_Incursion2Crafting: LazyLock<Vec<Incursion2CraftingRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/incursion2crafting.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| Incursion2CraftingRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#base_item_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#unknown24: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown28: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(28..28 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#icon_dds_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(32..32 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(40..40 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#description: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(48..48 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown56: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(56).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown57: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(57..57 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#glow_icon_dds_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(61..61 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#sound_effect: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(69..69 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                SoundEffectsRef::new(value as usize)
            },
            r#can_take_currency: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(85).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct Incursion2CraftingRow {
    pub r#id: String,
    pub r#base_item_type: BaseItemTypesRef,
    pub r#unknown24: i32,
    pub r#unknown28: i32,
    pub r#icon_dds_file: String,
    pub r#name: String,
    pub r#description: String,
    pub r#unknown56: bool,
    pub r#unknown57: i32,
    pub r#glow_icon_dds_file: String,
    pub r#sound_effect: SoundEffectsRef,
    pub r#can_take_currency: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct Incursion2CraftingRef(pub usize);

impl Deref for Incursion2CraftingRef {
    type Target = Incursion2CraftingRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_Incursion2Crafting[self.0]
    }
}

impl Incursion2CraftingRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static Incursion2CraftingRow {
        &TABLE_Incursion2Crafting[self.0]
    }
    pub fn get(&self) -> &'static Incursion2CraftingRow {
        &TABLE_Incursion2Crafting[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Incursion2Crafting.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static Incursion2CraftingRow)> {
        TABLE_Incursion2Crafting.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_Incursion2Crafting.iter() {
            black_box(row);
        }
    }
}
