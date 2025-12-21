#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_DamageCalculationTypes: LazyLock<Vec<DamageCalculationTypesRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/damagecalculationtypes.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| DamageCalculationTypesRow {
                r#id: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(0..0 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#physical_damage_stats: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(8..8 + 16).unwrap();
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
                        .map(|value| StatsRef::new(value as usize))
                        .collect()
                },
                r#fire_damage_stats: {
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
                    values
                        .into_iter()
                        .map(|value| StatsRef::new(value as usize))
                        .collect()
                },
                r#cold_damage_stats: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(40..40 + 16).unwrap();
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
                        .map(|value| StatsRef::new(value as usize))
                        .collect()
                },
                r#lightning_damage_stats: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(56..56 + 16).unwrap();
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
                        .map(|value| StatsRef::new(value as usize))
                        .collect()
                },
                r#chaos_damage_stats: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(72..72 + 16).unwrap();
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
                        .map(|value| StatsRef::new(value as usize))
                        .collect()
                },
                r#crit_chance_stat: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(88..88 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
                r#crit_bonus_stat: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(104..104 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
                r#always_crit_stat: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(120..120 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
                r#cannot_crit_stat: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(136..136 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
                r#stun_threshold_stat: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(152..152 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
                r#stun_duration_stat: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(168..168 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
                r#unknown184: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(184).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#cannot_block_stat: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(185..185 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
                r#knockback_stat: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(201..201 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
                r#knockback_on_crit_stat: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(217..217 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
                r#knockback_chance_stat: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(233..233 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
                r#text: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(249..249 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#unknown257: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(257..257 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    value
                },
                r#type: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(273..273 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#is_attack: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(277).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#unknown278: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(278..278 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
                r#unknown294: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(294..294 + 8).unwrap();
                    let value = cell_bytes.get_i64_le();
                    DamageCalculationTypesRef::new(value as usize)
                },
                r#unknown302: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(302).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#unknown303: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(303..303 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
                r#unknown319: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(319..319 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
                r#unknown335: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(335..335 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
                r#unknown351: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(351..351 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
                r#unknown367: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(367..367 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
                r#unknown383: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(383..383 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
                r#unknown399: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(399..399 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
                r#unknown415: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(415..415 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
                r#unknown431: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(431..431 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
                r#unknown447: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(447..447 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
                r#unknown463: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(463..463 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
                r#unknown479: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(479..479 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
                r#unknown495: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(495..495 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
                r#unknown511: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(511..511 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
                r#unknown527: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(527..527 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
                r#unknown543: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(543..543 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
                r#unknown559: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(559).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#unknown560: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(560..560 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
                r#unknown576: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(576..576 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
                r#unknown592: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(592..592 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
                r#unknown608: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(608..608 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
                r#unknown624: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(624..624 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
                r#unknown640: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(640..640 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
                r#unknown656: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(656..656 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct DamageCalculationTypesRow {
    pub r#id: String,
    pub r#physical_damage_stats: Vec<StatsRef>,
    pub r#fire_damage_stats: Vec<StatsRef>,
    pub r#cold_damage_stats: Vec<StatsRef>,
    pub r#lightning_damage_stats: Vec<StatsRef>,
    pub r#chaos_damage_stats: Vec<StatsRef>,
    pub r#crit_chance_stat: StatsRef,
    pub r#crit_bonus_stat: StatsRef,
    pub r#always_crit_stat: StatsRef,
    pub r#cannot_crit_stat: StatsRef,
    pub r#stun_threshold_stat: StatsRef,
    pub r#stun_duration_stat: StatsRef,
    pub r#unknown184: bool,
    pub r#cannot_block_stat: StatsRef,
    pub r#knockback_stat: StatsRef,
    pub r#knockback_on_crit_stat: StatsRef,
    pub r#knockback_chance_stat: StatsRef,
    pub r#text: String,
    pub r#unknown257: i64,
    pub r#type: i32,
    pub r#is_attack: bool,
    pub r#unknown278: StatsRef,
    pub r#unknown294: DamageCalculationTypesRef,
    pub r#unknown302: bool,
    pub r#unknown303: StatsRef,
    pub r#unknown319: StatsRef,
    pub r#unknown335: StatsRef,
    pub r#unknown351: StatsRef,
    pub r#unknown367: StatsRef,
    pub r#unknown383: StatsRef,
    pub r#unknown399: StatsRef,
    pub r#unknown415: StatsRef,
    pub r#unknown431: StatsRef,
    pub r#unknown447: StatsRef,
    pub r#unknown463: StatsRef,
    pub r#unknown479: StatsRef,
    pub r#unknown495: StatsRef,
    pub r#unknown511: StatsRef,
    pub r#unknown527: StatsRef,
    pub r#unknown543: StatsRef,
    pub r#unknown559: bool,
    pub r#unknown560: StatsRef,
    pub r#unknown576: StatsRef,
    pub r#unknown592: StatsRef,
    pub r#unknown608: StatsRef,
    pub r#unknown624: StatsRef,
    pub r#unknown640: StatsRef,
    pub r#unknown656: StatsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct DamageCalculationTypesRef(pub usize);

impl Deref for DamageCalculationTypesRef {
    type Target = DamageCalculationTypesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_DamageCalculationTypes[self.0]
    }
}

impl DamageCalculationTypesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static DamageCalculationTypesRow {
        &TABLE_DamageCalculationTypes[self.0]
    }
    pub fn get(&self) -> &'static DamageCalculationTypesRow {
        &TABLE_DamageCalculationTypes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_DamageCalculationTypes
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static DamageCalculationTypesRow)> {
        TABLE_DamageCalculationTypes
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
        for row in TABLE_DamageCalculationTypes.iter() {
            black_box(row);
        }
    }
}
