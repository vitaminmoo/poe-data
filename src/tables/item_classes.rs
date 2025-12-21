#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ItemClasses: LazyLock<Vec<ItemClassesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/itemclasses.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ItemClassesRow {
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
            r#item_class_category: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ItemClassCategoriesRef::new(value as usize)
            },
            r#removed_if_leaves_area: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(32).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown33: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(33..33 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#identify_achievements: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(49..49 + 16).unwrap();
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
            r#allocate_to_map_owner: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(65).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#always_allocate: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(66).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#can_have_veiled_mods: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(67).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#picked_up_quest: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(68..68 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                QuestFlagsRef::new(value as usize)
            },
            r#unknown84: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(84..84 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#always_show: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(88).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#can_be_corrupted: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(89).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#can_have_incubators: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(90).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#can_have_influence: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(91).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#can_be_double_corrupted: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(92).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#can_have_aspects: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(93).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#item_stance: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(94..94 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ItemStancesRef::new(value as usize)
            },
            r#can_scourge: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(110).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#can_upgrade_rarity: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(111).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown112: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(112).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#item_class_flags: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(113..113 + 16).unwrap();
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
            r#unmodfiable: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(129).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#can_be_fractured: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(130).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#equip_achievement: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(131..131 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                AchievementItemsRef::new(value as usize)
            },
            r#usable_in_map_device: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(147).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown148: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(148).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ItemClassesRow {
    pub r#id: String,
    pub r#name: String,
    pub r#item_class_category: ItemClassCategoriesRef,
    pub r#removed_if_leaves_area: bool,
    pub r#unknown33: (usize, usize),
    pub r#identify_achievements: Vec<AchievementItemsRef>,
    pub r#allocate_to_map_owner: bool,
    pub r#always_allocate: bool,
    pub r#can_have_veiled_mods: bool,
    pub r#picked_up_quest: QuestFlagsRef,
    pub r#unknown84: i32,
    pub r#always_show: bool,
    pub r#can_be_corrupted: bool,
    pub r#can_have_incubators: bool,
    pub r#can_have_influence: bool,
    pub r#can_be_double_corrupted: bool,
    pub r#can_have_aspects: bool,
    pub r#item_stance: ItemStancesRef,
    pub r#can_scourge: bool,
    pub r#can_upgrade_rarity: bool,
    pub r#unknown112: bool,
    pub r#item_class_flags: Vec<i32>,
    pub r#unmodfiable: bool,
    pub r#can_be_fractured: bool,
    pub r#equip_achievement: AchievementItemsRef,
    pub r#usable_in_map_device: bool,
    pub r#unknown148: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ItemClassesRef(pub usize);

impl Deref for ItemClassesRef {
    type Target = ItemClassesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ItemClasses[self.0]
    }
}

impl ItemClassesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ItemClassesRow {
        &TABLE_ItemClasses[self.0]
    }
    pub fn get(&self) -> &'static ItemClassesRow {
        &TABLE_ItemClasses[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ItemClasses.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ItemClassesRow)> {
        TABLE_ItemClasses
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn print_all_rows() {
        // Print all rows
        for row in TABLE_ItemClasses.iter() {
            println!("{:?}", row);
        }
    }
}
