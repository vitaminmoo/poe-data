#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MonsterTypes: LazyLock<Vec<MonsterTypesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/monstertypes.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| MonsterTypesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown8: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#is_summoned: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(12).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#armour: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(13..13 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#evasion: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(17..17 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#energy_shield_from_life: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(21..21 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#damage_spread: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(25..25 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#monster_resistances: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(29..29 + 16).unwrap();
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
                    .map(|value| MonsterResistancesRef::new(value as usize))
                    .collect()
            },
            r#unknown45: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(45).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown46: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(46..46 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MonsterTypesRow {
    pub r#id: String,
    pub r#unknown8: i32,
    pub r#is_summoned: bool,
    pub r#armour: i32,
    pub r#evasion: i32,
    pub r#energy_shield_from_life: i32,
    pub r#damage_spread: i32,
    pub r#monster_resistances: Vec<MonsterResistancesRef>,
    pub r#unknown45: bool,
    pub r#unknown46: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MonsterTypesRef(pub usize);

impl Deref for MonsterTypesRef {
    type Target = MonsterTypesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MonsterTypes[self.0]
    }
}

impl MonsterTypesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MonsterTypesRow {
        &TABLE_MonsterTypes[self.0]
    }
    pub fn get(&self) -> &'static MonsterTypesRow {
        &TABLE_MonsterTypes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MonsterTypes.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MonsterTypesRow)> {
        TABLE_MonsterTypes
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
        for row in TABLE_MonsterTypes.iter() {
            black_box(row);
        }
    }
}
