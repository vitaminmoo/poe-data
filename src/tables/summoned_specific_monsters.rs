#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_SummonedSpecificMonsters: LazyLock<Vec<SummonedSpecificMonstersRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/summonedspecificmonsters.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| SummonedSpecificMonstersRow {
                r#id: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(0..0 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#monster_varieties_key: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(4..4 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    MonsterVarietiesRef::new(value as usize)
                },
                r#unknown20: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(20..20 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown24: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(24..24 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    value
                },
                r#unknown40: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(40).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#unknown41: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(41).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#unknown42: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(42..42 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown46: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(46..46 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown50: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(50).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#unknown51: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(51..51 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    value
                },
                r#unknown67: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(67..67 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    value
                },
                r#unknown83: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(83..83 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown87: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(87).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#unknown88: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(88..88 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown92: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(92..92 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#unknown100: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(100).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#unknown101: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(101).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#unknown102: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(102..102 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct SummonedSpecificMonstersRow {
    pub r#id: i32,
    pub r#monster_varieties_key: MonsterVarietiesRef,
    pub r#unknown20: i32,
    pub r#unknown24: i64,
    pub r#unknown40: bool,
    pub r#unknown41: bool,
    pub r#unknown42: i32,
    pub r#unknown46: i32,
    pub r#unknown50: bool,
    pub r#unknown51: i64,
    pub r#unknown67: i64,
    pub r#unknown83: i32,
    pub r#unknown87: bool,
    pub r#unknown88: i32,
    pub r#unknown92: String,
    pub r#unknown100: bool,
    pub r#unknown101: bool,
    pub r#unknown102: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct SummonedSpecificMonstersRef(pub usize);

impl Deref for SummonedSpecificMonstersRef {
    type Target = SummonedSpecificMonstersRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_SummonedSpecificMonsters[self.0]
    }
}

impl SummonedSpecificMonstersRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static SummonedSpecificMonstersRow {
        &TABLE_SummonedSpecificMonsters[self.0]
    }
    pub fn get(&self) -> &'static SummonedSpecificMonstersRow {
        &TABLE_SummonedSpecificMonsters[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_SummonedSpecificMonsters
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static SummonedSpecificMonstersRow)> {
        TABLE_SummonedSpecificMonsters
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
        for row in TABLE_SummonedSpecificMonsters.iter() {
            black_box(row);
        }
    }
}
