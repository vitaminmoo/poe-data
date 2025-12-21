#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_WorldAreaLeagueChances: LazyLock<Vec<WorldAreaLeagueChancesRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/worldarealeaguechances.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| WorldAreaLeagueChancesRow {
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
                r#unknown12: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(12..12 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown16: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(16..16 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown20: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(20..20 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown24: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(24..24 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown28: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(28..28 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown32: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(32..32 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown36: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(36..36 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown40: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(40..40 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown44: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(44..44 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown48: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(48..48 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown52: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(52..52 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown56: {
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
                r#abyss_chance: {
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
                r#unknown80: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(80..80 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown84: {
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
                r#unknown92: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(92..92 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown96: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(96..96 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#stone_circles_chance: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(100..100 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct WorldAreaLeagueChancesRow {
    pub r#id: String,
    pub r#unknown8: i32,
    pub r#unknown12: i32,
    pub r#unknown16: i32,
    pub r#unknown20: i32,
    pub r#unknown24: i32,
    pub r#unknown28: i32,
    pub r#unknown32: i32,
    pub r#unknown36: i32,
    pub r#unknown40: i32,
    pub r#unknown44: i32,
    pub r#unknown48: i32,
    pub r#unknown52: i32,
    pub r#unknown56: i32,
    pub r#unknown60: i32,
    pub r#unknown64: i32,
    pub r#abyss_chance: i32,
    pub r#unknown72: i32,
    pub r#unknown76: i32,
    pub r#unknown80: i32,
    pub r#unknown84: i32,
    pub r#unknown88: i32,
    pub r#unknown92: i32,
    pub r#unknown96: i32,
    pub r#stone_circles_chance: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct WorldAreaLeagueChancesRef(pub usize);

impl Deref for WorldAreaLeagueChancesRef {
    type Target = WorldAreaLeagueChancesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_WorldAreaLeagueChances[self.0]
    }
}

impl WorldAreaLeagueChancesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static WorldAreaLeagueChancesRow {
        &TABLE_WorldAreaLeagueChances[self.0]
    }
    pub fn get(&self) -> &'static WorldAreaLeagueChancesRow {
        &TABLE_WorldAreaLeagueChances[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_WorldAreaLeagueChances
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static WorldAreaLeagueChancesRow)> {
        TABLE_WorldAreaLeagueChances
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
        for row in TABLE_WorldAreaLeagueChances.iter() {
            black_box(row);
        }
    }
}
