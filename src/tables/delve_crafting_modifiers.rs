#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_DelveCraftingModifiers: LazyLock<Vec<DelveCraftingModifiersRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/delvecraftingmodifiers.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| DelveCraftingModifiersRow {
                r#base_item_type: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(0..0 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    BaseItemTypesRef::new(value as usize)
                },
                r#added_mods: {
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
                        .map(|value| ModsRef::new(value as usize))
                        .collect()
                },
                r#negative_weight_tags: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(32..32 + 16).unwrap();
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
                        .map(|value| TagsRef::new(value as usize))
                        .collect()
                },
                r#negative_weight_values: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(48..48 + 16).unwrap();
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
                r#forced_add_mods: {
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
                        .map(|value| ModsRef::new(value as usize))
                        .collect()
                },
                r#forbidden_delve_crafting_tags: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(80..80 + 16).unwrap();
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
                        .map(|value| DelveCraftingTagsRef::new(value as usize))
                        .collect()
                },
                r#allowed_delve_crafting_tags: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(96..96 + 16).unwrap();
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
                        .map(|value| DelveCraftingTagsRef::new(value as usize))
                        .collect()
                },
                r#can_mirror_item: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(112).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#corrupted_essence_chance: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(113..113 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#can_improve_quality: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(117).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#has_lucky_rolls: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(118).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#unknown119: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(119).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#sell_price_mods: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(120..120 + 16).unwrap();
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
                        .map(|value| ModsRef::new(value as usize))
                        .collect()
                },
                r#can_roll_white_sockets: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(136).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#weight_tags: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(137..137 + 16).unwrap();
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
                        .map(|value| TagsRef::new(value as usize))
                        .collect()
                },
                r#weight_values: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(153..153 + 16).unwrap();
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
                r#delve_crafting_modifier_descriptions: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(169..169 + 16).unwrap();
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
                        .map(|value| DelveCraftingModifierDescriptionsRef::new(value as usize))
                        .collect()
                },
                r#blocked_delve_crafting_modifier_descriptions: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(185..185 + 16).unwrap();
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
                        .map(|value| DelveCraftingModifierDescriptionsRef::new(value as usize))
                        .collect()
                },
                r#unknown201: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(201).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#unknown202: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(202).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#unknown203: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(203..203 + 16).unwrap();
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
                r#unknown219: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(219..219 + 16).unwrap();
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
            })
            .collect()
    });

#[derive(Debug)]
pub struct DelveCraftingModifiersRow {
    pub r#base_item_type: BaseItemTypesRef,
    pub r#added_mods: Vec<ModsRef>,
    pub r#negative_weight_tags: Vec<TagsRef>,
    pub r#negative_weight_values: Vec<i32>,
    pub r#forced_add_mods: Vec<ModsRef>,
    pub r#forbidden_delve_crafting_tags: Vec<DelveCraftingTagsRef>,
    pub r#allowed_delve_crafting_tags: Vec<DelveCraftingTagsRef>,
    pub r#can_mirror_item: bool,
    pub r#corrupted_essence_chance: i32,
    pub r#can_improve_quality: bool,
    pub r#has_lucky_rolls: bool,
    pub r#unknown119: bool,
    pub r#sell_price_mods: Vec<ModsRef>,
    pub r#can_roll_white_sockets: bool,
    pub r#weight_tags: Vec<TagsRef>,
    pub r#weight_values: Vec<i32>,
    pub r#delve_crafting_modifier_descriptions: Vec<DelveCraftingModifierDescriptionsRef>,
    pub r#blocked_delve_crafting_modifier_descriptions: Vec<DelveCraftingModifierDescriptionsRef>,
    pub r#unknown201: bool,
    pub r#unknown202: bool,
    pub r#unknown203: Vec<i32>,
    pub r#unknown219: Vec<i32>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct DelveCraftingModifiersRef(pub usize);

impl Deref for DelveCraftingModifiersRef {
    type Target = DelveCraftingModifiersRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_DelveCraftingModifiers[self.0]
    }
}

impl DelveCraftingModifiersRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static DelveCraftingModifiersRow {
        &TABLE_DelveCraftingModifiers[self.0]
    }
    pub fn get(&self) -> &'static DelveCraftingModifiersRow {
        &TABLE_DelveCraftingModifiers[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_DelveCraftingModifiers
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static DelveCraftingModifiersRow)> {
        TABLE_DelveCraftingModifiers
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
        for row in TABLE_DelveCraftingModifiers.iter() {
            black_box(row);
        }
    }
}
