#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_PlayerConditions: LazyLock<Vec<PlayerConditionsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/playerconditions.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| PlayerConditionsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#buff_definitions_keys: {
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
                values.into_iter().map(|value| BuffDefinitionsRef::new(value as usize)).collect()
            },
            r#unknown24: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(24).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#buff_stacks: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(25..25 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#characters_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(29..29 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                CharactersRef::new(value as usize)
            },
            r#stats_keys: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(45..45 + 16).unwrap();
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
            r#unknown61: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(61).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#stat_value: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(62..62 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown66: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(66..66 + 16).unwrap();
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
            r#unknown82: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(82).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct PlayerConditionsRow {
    pub r#id: String,
    pub r#buff_definitions_keys: Vec<BuffDefinitionsRef>,
    pub r#unknown24: bool,
    pub r#buff_stacks: i32,
    pub r#characters_key: CharactersRef,
    pub r#stats_keys: Vec<StatsRef>,
    pub r#unknown61: bool,
    pub r#stat_value: i32,
    pub r#unknown66: Vec<i64>,
    pub r#unknown82: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct PlayerConditionsRef(pub usize);

impl Deref for PlayerConditionsRef {
    type Target = PlayerConditionsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_PlayerConditions[self.0]
    }
}

impl PlayerConditionsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static PlayerConditionsRow {
        &TABLE_PlayerConditions[self.0]
    }
    pub fn get(&self) -> &'static PlayerConditionsRow {
        &TABLE_PlayerConditions[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_PlayerConditions.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static PlayerConditionsRow)> {
        TABLE_PlayerConditions.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_PlayerConditions.iter() {
            black_box(row);
        }
    }
}
