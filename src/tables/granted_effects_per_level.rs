#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_GrantedEffectsPerLevel: LazyLock<Vec<GrantedEffectsPerLevelRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/grantedeffectsperlevel.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| GrantedEffectsPerLevelRow {
                r#granted_effect: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(0..0 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    GrantedEffectsRef::new(value as usize)
                },
                r#level: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(16..16 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#cost_multiplier: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(20..20 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#stored_uses: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(24..24 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#cooldown: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(28..28 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#cooldown_bypass_type: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(32..32 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    CooldownBypassTypes::from_repr(value as usize)
                },
                r#vaal_souls: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(36..36 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#vaal_stored_uses: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(40..40 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#cooldown_group: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(44..44 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#pv_p_damage_multiplier: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(48..48 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#soul_gain_prevention_duration: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(52..52 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#attack_speed_multiplier: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(56..56 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown60: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(60..60 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown64: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(64..64 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown68: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(68..68 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown72: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(72..72 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown76: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(76..76 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#attack_time: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(80..80 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#reservation: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(84..84 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown88: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(88..88 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#cost_amounts: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(92..92 + 16).unwrap();
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
                r#actor_level: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(108..108 + 4).unwrap();
                    let value = cell_bytes.get_f32_le();
                    value
                },
                r#effect_on_player: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(112..112 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct GrantedEffectsPerLevelRow {
    pub r#granted_effect: GrantedEffectsRef,
    pub r#level: i32,
    pub r#cost_multiplier: i32,
    pub r#stored_uses: i32,
    pub r#cooldown: i32,
    pub r#cooldown_bypass_type: Option<CooldownBypassTypes>,
    pub r#vaal_souls: i32,
    pub r#vaal_stored_uses: i32,
    pub r#cooldown_group: i32,
    pub r#pv_p_damage_multiplier: i32,
    pub r#soul_gain_prevention_duration: i32,
    pub r#attack_speed_multiplier: i32,
    pub r#unknown60: i32,
    pub r#unknown64: i32,
    pub r#unknown68: i32,
    pub r#unknown72: i32,
    pub r#unknown76: i32,
    pub r#attack_time: i32,
    pub r#reservation: i32,
    pub r#unknown88: i32,
    pub r#cost_amounts: Vec<i32>,
    pub r#actor_level: f32,
    pub r#effect_on_player: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct GrantedEffectsPerLevelRef(pub usize);

impl Deref for GrantedEffectsPerLevelRef {
    type Target = GrantedEffectsPerLevelRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_GrantedEffectsPerLevel[self.0]
    }
}

impl GrantedEffectsPerLevelRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static GrantedEffectsPerLevelRow {
        &TABLE_GrantedEffectsPerLevel[self.0]
    }
    pub fn get(&self) -> &'static GrantedEffectsPerLevelRow {
        &TABLE_GrantedEffectsPerLevel[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_GrantedEffectsPerLevel
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static GrantedEffectsPerLevelRow)> {
        TABLE_GrantedEffectsPerLevel
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
        for row in TABLE_GrantedEffectsPerLevel.iter() {
            println!("{:?}", row);
        }
    }
}
