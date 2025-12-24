#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_CraftingBenchOptions: LazyLock<Vec<CraftingBenchOptionsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/craftingbenchoptions.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| CraftingBenchOptionsRow {
            r#hideout_npc: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                HideoutNPCsRef::new(value as usize)
            },
            r#order: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#add_mod: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ModsRef::new(value as usize)
            },
            r#cost_base_item_types: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(36..36 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| BaseItemTypesRef::new(value as usize)).collect()
            },
            r#cost_values: {
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
            r#required_level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(68..68 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(72..72 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#crafting_bench_custom_action: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(80..80 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                CraftingBenchCustomActions::from_repr(value as usize)
            },
            r#item_classes: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(84..84 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| ItemClassesRef::new(value as usize)).collect()
            },
            r#links: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(100..100 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#socket_colours: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(104..104 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#sockets: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(112..112 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#item_quantity: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(116..116 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown120: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(120..120 + 16).unwrap();
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
            r#description: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(136..136 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#is_disabled: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(144).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#is_area_option: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(145).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#recipe_ids: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(146..146 + 16).unwrap();
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
            r#tier: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(162..162 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#crafting_item_class_categories: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(166..166 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| CraftingItemClassCategoriesRef::new(value as usize)).collect()
            },
            r#unknown182: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(182..182 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unlock_category: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(186..186 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                CraftingBenchUnlockCategoriesRef::new(value as usize)
            },
            r#unveils_required: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(202..202 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unveils_required2: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(206..206 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown210: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(210..210 + 16).unwrap();
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
            },
            r#kalandra_achievement: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(226..226 + 16).unwrap();
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
            r#unknown242: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(242..242 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown246: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(246..246 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#veiled_mod: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(250..250 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ModsRef::new(value as usize)
            },
            r#add_enchantment: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(266..266 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ModsRef::new(value as usize)
            },
            r#sort_category: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(282..282 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                CraftingBenchSortCategoriesRef::new(value as usize)
            },
            r#mod_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(298..298 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ModTypeRef::new(value as usize)
            },
            r#unknown314: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(314).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown315: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(315..315 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#stat1: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(319..319 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#stat2: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(335..335 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#stat3: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(351..351 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct CraftingBenchOptionsRow {
    pub r#hideout_npc: HideoutNPCsRef,
    pub r#order: i32,
    pub r#add_mod: ModsRef,
    pub r#cost_base_item_types: Vec<BaseItemTypesRef>,
    pub r#cost_values: Vec<i32>,
    pub r#required_level: i32,
    pub r#name: String,
    pub r#crafting_bench_custom_action: Option<CraftingBenchCustomActions>,
    pub r#item_classes: Vec<ItemClassesRef>,
    pub r#links: i32,
    pub r#socket_colours: String,
    pub r#sockets: i32,
    pub r#item_quantity: i32,
    pub r#unknown120: Vec<i32>,
    pub r#description: String,
    pub r#is_disabled: bool,
    pub r#is_area_option: bool,
    pub r#recipe_ids: Vec<i32>,
    pub r#tier: i32,
    pub r#crafting_item_class_categories: Vec<CraftingItemClassCategoriesRef>,
    pub r#unknown182: i32,
    pub r#unlock_category: CraftingBenchUnlockCategoriesRef,
    pub r#unveils_required: i32,
    pub r#unveils_required2: i32,
    pub r#unknown210: Vec<i64>,
    pub r#kalandra_achievement: Vec<AchievementItemsRef>,
    pub r#unknown242: i32,
    pub r#unknown246: i32,
    pub r#veiled_mod: ModsRef,
    pub r#add_enchantment: ModsRef,
    pub r#sort_category: CraftingBenchSortCategoriesRef,
    pub r#mod_type: ModTypeRef,
    pub r#unknown314: bool,
    pub r#unknown315: i32,
    pub r#stat1: StatsRef,
    pub r#stat2: StatsRef,
    pub r#stat3: StatsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct CraftingBenchOptionsRef(pub usize);

impl Deref for CraftingBenchOptionsRef {
    type Target = CraftingBenchOptionsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_CraftingBenchOptions[self.0]
    }
}

impl CraftingBenchOptionsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static CraftingBenchOptionsRow {
        &TABLE_CraftingBenchOptions[self.0]
    }
    pub fn get(&self) -> &'static CraftingBenchOptionsRow {
        &TABLE_CraftingBenchOptions[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_CraftingBenchOptions.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static CraftingBenchOptionsRow)> {
        TABLE_CraftingBenchOptions.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_CraftingBenchOptions.iter() {
            black_box(row);
        }
    }
}
