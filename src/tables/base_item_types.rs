#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_BaseItemTypes: LazyLock<Vec<BaseItemTypesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/baseitemtypes.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| BaseItemTypesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#item_class: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ItemClassesRef::new(value as usize)
            },
            r#width: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#height: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(28..28 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(32..32 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#inherits_from: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(40..40 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#drop_level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(48..48 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#flavour_text: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(52..52 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                FlavourTextRef::new(value as usize)
            },
            r#implicit_mods: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(68..68 + 16).unwrap();
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
            r#size_on_ground: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(84..84 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#sound_effect: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(88..88 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                SoundEffectsRef::new(value as usize)
            },
            r#tags: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(104..104 + 16).unwrap();
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
            r#mod_domain: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(120..120 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                ModDomains::from_repr(value as usize)
            },
            r#item_visual_identity: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(124..124 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ItemVisualIdentityRef::new(value as usize)
            },
            r#hash32: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(140..140 + 4).unwrap();
                let value = cell_bytes.get_u32_le();
                value
            },
            r#vendor_recipe_achievement_items: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(144..144 + 16).unwrap();
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
            r#inflection: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(160..160 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#equip_achievement_item: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(168..168 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                AchievementItemsRef::new(value as usize)
            },
            r#is_corrupted: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(184).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#identify_achievement_items: {
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
                    .map(|value| AchievementItemsRef::new(value as usize))
                    .collect()
            },
            r#identify_magic_achievement_items: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(201..201 + 16).unwrap();
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
            r#fragment_base_item_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(217..217 + 8).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#unknown225: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(225).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#uncut_gem_sound_effect: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(226..226 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                SoundEffectsRef::new(value as usize)
            },
            r#unknown242: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(242..242 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown258: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(258).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unmodifiable: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(259).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#achievement: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(260..260 + 16).unwrap();
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
            r#shop_tag: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(276..276 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ShopTagRef::new(value as usize)
            },
            r#unknown292: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(292..292 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct BaseItemTypesRow {
    pub r#id: String,
    pub r#item_class: ItemClassesRef,
    pub r#width: i32,
    pub r#height: i32,
    pub r#name: String,
    pub r#inherits_from: String,
    pub r#drop_level: i32,
    pub r#flavour_text: FlavourTextRef,
    pub r#implicit_mods: Vec<ModsRef>,
    pub r#size_on_ground: i32,
    pub r#sound_effect: SoundEffectsRef,
    pub r#tags: Vec<TagsRef>,
    pub r#mod_domain: Option<ModDomains>,
    pub r#item_visual_identity: ItemVisualIdentityRef,
    pub r#hash32: u32,
    pub r#vendor_recipe_achievement_items: Vec<AchievementItemsRef>,
    pub r#inflection: String,
    pub r#equip_achievement_item: AchievementItemsRef,
    pub r#is_corrupted: bool,
    pub r#identify_achievement_items: Vec<AchievementItemsRef>,
    pub r#identify_magic_achievement_items: Vec<AchievementItemsRef>,
    pub r#fragment_base_item_type: BaseItemTypesRef,
    pub r#unknown225: bool,
    pub r#uncut_gem_sound_effect: SoundEffectsRef,
    pub r#unknown242: i64,
    pub r#unknown258: bool,
    pub r#unmodifiable: bool,
    pub r#achievement: Vec<AchievementItemsRef>,
    pub r#shop_tag: ShopTagRef,
    pub r#unknown292: i64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BaseItemTypesRef(pub usize);

impl Deref for BaseItemTypesRef {
    type Target = BaseItemTypesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_BaseItemTypes[self.0]
    }
}

impl BaseItemTypesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BaseItemTypesRow {
        &TABLE_BaseItemTypes[self.0]
    }
    pub fn get(&self) -> &'static BaseItemTypesRow {
        &TABLE_BaseItemTypes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BaseItemTypes.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BaseItemTypesRow)> {
        TABLE_BaseItemTypes
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
        for row in TABLE_BaseItemTypes.iter() {
            println!("{:?}", row);
        }
    }
}
