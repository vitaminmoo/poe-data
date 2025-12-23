#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_CurrencyItems: LazyLock<Vec<CurrencyItemsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/currencyitems.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| CurrencyItemsRow {
            r#base_item_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#stack_size: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#currency_use_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#action: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(24..24 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#directions: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(32..32 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#full_stack_base_item_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#description: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(56..56 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#usage_achievement_items: {
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
            r#scroll: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(80).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#possession_achievement_item: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(81..81 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                AchievementItemsRef::new(value as usize)
            },
            r#unknown97: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(97..97 + 16).unwrap();
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
            r#unknown113: {
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
            r#currency_tab_stack_size: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(129..129 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#x_box_directions: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(133..133 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown141: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(141..141 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#modify_maps_achievements: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(145..145 + 16).unwrap();
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
            r#modify_contracts_achievements: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(161..161 + 16).unwrap();
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
            r#combine_achievements: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(177..177 + 16).unwrap();
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
            r#changed_for_hardmode: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(193).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#description_hardmode: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(194..194 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#is_gold: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(202).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#usage_hint: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(203..203 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown211: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(211).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown212: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(212).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown213: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(213..213 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct CurrencyItemsRow {
    pub r#base_item_type: BaseItemTypesRef,
    pub r#stack_size: i32,
    pub r#currency_use_type: i32,
    pub r#action: String,
    pub r#directions: String,
    pub r#full_stack_base_item_type: BaseItemTypesRef,
    pub r#description: String,
    pub r#usage_achievement_items: Vec<AchievementItemsRef>,
    pub r#scroll: bool,
    pub r#possession_achievement_item: AchievementItemsRef,
    pub r#unknown97: Vec<i64>,
    pub r#unknown113: Vec<i32>,
    pub r#currency_tab_stack_size: i32,
    pub r#x_box_directions: String,
    pub r#unknown141: i32,
    pub r#modify_maps_achievements: Vec<AchievementItemsRef>,
    pub r#modify_contracts_achievements: Vec<AchievementItemsRef>,
    pub r#combine_achievements: Vec<AchievementItemsRef>,
    pub r#changed_for_hardmode: bool,
    pub r#description_hardmode: String,
    pub r#is_gold: bool,
    pub r#usage_hint: String,
    pub r#unknown211: bool,
    pub r#unknown212: bool,
    pub r#unknown213: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct CurrencyItemsRef(pub usize);

impl Deref for CurrencyItemsRef {
    type Target = CurrencyItemsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_CurrencyItems[self.0]
    }
}

impl CurrencyItemsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static CurrencyItemsRow {
        &TABLE_CurrencyItems[self.0]
    }
    pub fn get(&self) -> &'static CurrencyItemsRow {
        &TABLE_CurrencyItems[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_CurrencyItems.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static CurrencyItemsRow)> {
        TABLE_CurrencyItems.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_CurrencyItems.iter() {
            black_box(row);
        }
    }
}
