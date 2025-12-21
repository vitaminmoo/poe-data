#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_SkillGems: LazyLock<Vec<SkillGemsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/skillgems.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| SkillGemsRow {
            r#base_item_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#strength_requirement_percent: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#dexterity_requirement_percent: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#intelligence_requirement_percent: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#vaal_variant_base_item_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(28..28 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#is_vaal_variant: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(44).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#global_skill_level_stat: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(45..45 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#gem_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(61..61 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown65: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(65).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown66: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(66).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#awakened: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(67..67 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#gem_colour: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(83..83 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#min_level_req: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(87..87 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#item_experience_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(91..91 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ItemExperienceTypesRef::new(value as usize)
            },
            r#crafting_types: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(107..107 + 16).unwrap();
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
                    .map(|value| SkillCraftingDataRef::new(value as usize))
                    .collect()
            },
            r#mtx_slot_types: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(123..123 + 16).unwrap();
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
                    .map(|value| MicrotransactionSkillGemEffectSlotTypesRef::new(value as usize))
                    .collect()
            },
            r#unknown139: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(139).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#gem_effects: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(140..140 + 16).unwrap();
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
                    .map(|value| GemEffectsRef::new(value as usize))
                    .collect()
            },
            r#unknown156: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(156).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#tutorial_video: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(157..157 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#ui_image: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(165..165 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown173: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(173..173 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#crafting_level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(177..177 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown181: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(181).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown182: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(182..182 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown186: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(186).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown187: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(187..187 + 16).unwrap();
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
            r#tier: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(203..203 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct SkillGemsRow {
    pub r#base_item_type: BaseItemTypesRef,
    pub r#strength_requirement_percent: i32,
    pub r#dexterity_requirement_percent: i32,
    pub r#intelligence_requirement_percent: i32,
    pub r#vaal_variant_base_item_type: BaseItemTypesRef,
    pub r#is_vaal_variant: bool,
    pub r#global_skill_level_stat: StatsRef,
    pub r#gem_type: i32,
    pub r#unknown65: bool,
    pub r#unknown66: bool,
    pub r#awakened: i64,
    pub r#gem_colour: i32,
    pub r#min_level_req: i32,
    pub r#item_experience_type: ItemExperienceTypesRef,
    pub r#crafting_types: Vec<SkillCraftingDataRef>,
    pub r#mtx_slot_types: Vec<MicrotransactionSkillGemEffectSlotTypesRef>,
    pub r#unknown139: bool,
    pub r#gem_effects: Vec<GemEffectsRef>,
    pub r#unknown156: bool,
    pub r#tutorial_video: String,
    pub r#ui_image: String,
    pub r#unknown173: i32,
    pub r#crafting_level: i32,
    pub r#unknown181: bool,
    pub r#unknown182: i32,
    pub r#unknown186: bool,
    pub r#unknown187: Vec<i64>,
    pub r#tier: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct SkillGemsRef(pub usize);

impl Deref for SkillGemsRef {
    type Target = SkillGemsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_SkillGems[self.0]
    }
}

impl SkillGemsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static SkillGemsRow {
        &TABLE_SkillGems[self.0]
    }
    pub fn get(&self) -> &'static SkillGemsRow {
        &TABLE_SkillGems[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_SkillGems.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static SkillGemsRow)> {
        TABLE_SkillGems
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
        for row in TABLE_SkillGems.iter() {
            black_box(row);
        }
    }
}
