#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_GrantedEffectStatSetsPerLevel: LazyLock<Vec<GrantedEffectStatSetsPerLevelRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/grantedeffectstatsetsperlevel.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| GrantedEffectStatSetsPerLevelRow {
            r#stat_set: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                GrantedEffectStatSetsRef::new(value as usize)
            },
            r#gem_level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#spell_crit_chance: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#attack_crit_chance: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#base_resolved_values: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(28..28 + 16).unwrap();
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
            r#additional_stats_values: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(44..44 + 16).unwrap();
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
            r#granted_effects: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(60..60 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| GrantedEffectsRef::new(value as usize)).collect()
            },
            r#unknown76: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(76).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown77: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(77..77 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#additional_flags: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(81..81 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| StatsRef::new(value as usize)).collect()
            },
            r#float_stats: {
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
                values.into_iter().map(|value| StatsRef::new(value as usize)).collect()
            },
            r#interpolation_bases: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(113..113 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| StatsRef::new(value as usize)).collect()
            },
            r#additional_stats: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(129..129 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| StatsRef::new(value as usize)).collect()
            },
            r#stat_interpolations: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(145..145 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 4)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i32_le())
                    .collect::<Vec<i32>>();
                values.into_iter().map(|value| StatInterpolationTypes::from_repr(value as usize)).collect()
            },
            r#float_stats_values: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(161..161 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 4)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_f32_le())
                    .collect::<Vec<f32>>();
                values
            },
            r#actor_level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(177..177 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#base_multiplier: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(181..181 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct GrantedEffectStatSetsPerLevelRow {
    pub r#stat_set: GrantedEffectStatSetsRef,
    pub r#gem_level: i32,
    pub r#spell_crit_chance: i32,
    pub r#attack_crit_chance: i32,
    pub r#base_resolved_values: Vec<i32>,
    pub r#additional_stats_values: Vec<i32>,
    pub r#granted_effects: Vec<GrantedEffectsRef>,
    pub r#unknown76: bool,
    pub r#unknown77: i32,
    pub r#additional_flags: Vec<StatsRef>,
    pub r#float_stats: Vec<StatsRef>,
    pub r#interpolation_bases: Vec<StatsRef>,
    pub r#additional_stats: Vec<StatsRef>,
    pub r#stat_interpolations: Vec<Option<StatInterpolationTypes>>,
    pub r#float_stats_values: Vec<f32>,
    pub r#actor_level: f32,
    pub r#base_multiplier: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct GrantedEffectStatSetsPerLevelRef(pub usize);

impl Deref for GrantedEffectStatSetsPerLevelRef {
    type Target = GrantedEffectStatSetsPerLevelRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_GrantedEffectStatSetsPerLevel[self.0]
    }
}

impl GrantedEffectStatSetsPerLevelRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static GrantedEffectStatSetsPerLevelRow {
        &TABLE_GrantedEffectStatSetsPerLevel[self.0]
    }
    pub fn get(&self) -> &'static GrantedEffectStatSetsPerLevelRow {
        &TABLE_GrantedEffectStatSetsPerLevel[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_GrantedEffectStatSetsPerLevel.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static GrantedEffectStatSetsPerLevelRow)> {
        TABLE_GrantedEffectStatSetsPerLevel.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_GrantedEffectStatSetsPerLevel.iter() {
            black_box(row);
        }
    }
}
