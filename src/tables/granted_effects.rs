#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_GrantedEffects: LazyLock<Vec<GrantedEffectsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/grantedeffects.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| GrantedEffectsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#is_support: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(8).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#allowed_active_skill_types: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(9..9 + 16).unwrap();
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
                    .map(|value| ActiveSkillTypeRef::new(value as usize))
                    .collect()
            },
            r#support_gem_letter: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(25..25 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#added_active_skill_types: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(33..33 + 16).unwrap();
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
                    .map(|value| ActiveSkillTypeRef::new(value as usize))
                    .collect()
            },
            r#excluded_active_skill_types: {
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
                    .map(|value| ActiveSkillTypeRef::new(value as usize))
                    .collect()
            },
            r#supports_gems_only: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(65).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#hash32: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(66..66 + 4).unwrap();
                let value = cell_bytes.get_u32_le();
                value
            },
            r#cannot_be_supported: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(70).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#life_leech: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(71..71 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#cast_time: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(75..75 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#active_skill: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(79..79 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ActiveSkillsRef::new(value as usize)
            },
            r#ignore_minion_types: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(95).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#cooldown_not_recover_during_active: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(96).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#added_minion_active_skill_types: {
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
                    .into_iter()
                    .map(|value| ActiveSkillTypeRef::new(value as usize))
                    .collect()
            },
            r#animation: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(113..113 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                AnimationRef::new(value as usize)
            },
            r#multi_part_achievement: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(129..129 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MultiPartAchievementsRef::new(value as usize)
            },
            r#unknown145: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(145).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#regular_variant: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(146..146 + 8).unwrap();
                let value = cell_bytes.get_i64_le();
                GrantedEffectsRef::new(value as usize)
            },
            r#unknown154: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(154).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown155: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(155..155 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown159: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(159..159 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown163: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(163..163 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown167: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(167).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#stat_set: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(168..168 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                GrantedEffectStatSetsRef::new(value as usize)
            },
            r#additional_stat_sets: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(184..184 + 16).unwrap();
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
                    .map(|value| GrantedEffectStatSetsRef::new(value as usize))
                    .collect()
            },
            r#audio: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(200..200 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#cost_types: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(208..208 + 16).unwrap();
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
                    .map(|value| CostTypesRef::new(value as usize))
                    .collect()
            },
            r#unknown224: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(224).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown225: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(225..225 + 16).unwrap();
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
            r#unknown241: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(241..241 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#support_weapon_restrictions: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(245..245 + 16).unwrap();
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
                    .map(|value| ItemClassesRef::new(value as usize))
                    .collect()
            },
            r#unknown261: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(261..261 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#minion_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(269..269 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MinionTypeRef::new(value as usize)
            },
            r#unknown285: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(285..285 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown289: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(289..289 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown293: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(293..293 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct GrantedEffectsRow {
    pub r#id: String,
    pub r#is_support: bool,
    pub r#allowed_active_skill_types: Vec<ActiveSkillTypeRef>,
    pub r#support_gem_letter: String,
    pub r#added_active_skill_types: Vec<ActiveSkillTypeRef>,
    pub r#excluded_active_skill_types: Vec<ActiveSkillTypeRef>,
    pub r#supports_gems_only: bool,
    pub r#hash32: u32,
    pub r#cannot_be_supported: bool,
    pub r#life_leech: i32,
    pub r#cast_time: i32,
    pub r#active_skill: ActiveSkillsRef,
    pub r#ignore_minion_types: bool,
    pub r#cooldown_not_recover_during_active: bool,
    pub r#added_minion_active_skill_types: Vec<ActiveSkillTypeRef>,
    pub r#animation: AnimationRef,
    pub r#multi_part_achievement: MultiPartAchievementsRef,
    pub r#unknown145: bool,
    pub r#regular_variant: GrantedEffectsRef,
    pub r#unknown154: bool,
    pub r#unknown155: i32,
    pub r#unknown159: i32,
    pub r#unknown163: i32,
    pub r#unknown167: bool,
    pub r#stat_set: GrantedEffectStatSetsRef,
    pub r#additional_stat_sets: Vec<GrantedEffectStatSetsRef>,
    pub r#audio: String,
    pub r#cost_types: Vec<CostTypesRef>,
    pub r#unknown224: bool,
    pub r#unknown225: Vec<i32>,
    pub r#unknown241: i32,
    pub r#support_weapon_restrictions: Vec<ItemClassesRef>,
    pub r#unknown261: String,
    pub r#minion_type: MinionTypeRef,
    pub r#unknown285: f32,
    pub r#unknown289: f32,
    pub r#unknown293: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct GrantedEffectsRef(pub usize);

impl Deref for GrantedEffectsRef {
    type Target = GrantedEffectsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_GrantedEffects[self.0]
    }
}

impl GrantedEffectsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static GrantedEffectsRow {
        &TABLE_GrantedEffects[self.0]
    }
    pub fn get(&self) -> &'static GrantedEffectsRow {
        &TABLE_GrantedEffects[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_GrantedEffects
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static GrantedEffectsRow)> {
        TABLE_GrantedEffects
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
        for row in TABLE_GrantedEffects.iter() {
            black_box(row);
        }
    }
}
