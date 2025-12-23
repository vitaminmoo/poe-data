#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_BetrayalUpgrades: LazyLock<Vec<BetrayalUpgradesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/betrayalupgrades.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| BetrayalUpgradesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#description: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#mods_key: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| ModsRef::new(value as usize)).collect()
            },
            r#art_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(40..40 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#betrayal_upgrade_slots_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(48..48 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown52: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(52..52 + 16).unwrap();
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
            r#item_visual_identity_key0: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(68..68 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ItemVisualIdentityRef::new(value as usize)
            },
            r#item_visual_identity_key1: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(84..84 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ItemVisualIdentityRef::new(value as usize)
            },
            r#granted_effects_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(100..100 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                GrantedEffectsRef::new(value as usize)
            },
            r#unknown116: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(116..116 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#item_classes_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(120..120 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ItemClassesRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct BetrayalUpgradesRow {
    pub r#id: String,
    pub r#name: String,
    pub r#description: String,
    pub r#mods_key: Vec<ModsRef>,
    pub r#art_file: String,
    pub r#betrayal_upgrade_slots_key: i32,
    pub r#unknown52: Vec<i32>,
    pub r#item_visual_identity_key0: ItemVisualIdentityRef,
    pub r#item_visual_identity_key1: ItemVisualIdentityRef,
    pub r#granted_effects_key: GrantedEffectsRef,
    pub r#unknown116: i32,
    pub r#item_classes_key: ItemClassesRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BetrayalUpgradesRef(pub usize);

impl Deref for BetrayalUpgradesRef {
    type Target = BetrayalUpgradesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_BetrayalUpgrades[self.0]
    }
}

impl BetrayalUpgradesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BetrayalUpgradesRow {
        &TABLE_BetrayalUpgrades[self.0]
    }
    pub fn get(&self) -> &'static BetrayalUpgradesRow {
        &TABLE_BetrayalUpgrades[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BetrayalUpgrades.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BetrayalUpgradesRow)> {
        TABLE_BetrayalUpgrades.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_BetrayalUpgrades.iter() {
            black_box(row);
        }
    }
}
