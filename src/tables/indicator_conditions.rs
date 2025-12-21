#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_IndicatorConditions: LazyLock<Vec<IndicatorConditionsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/indicatorconditions.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| IndicatorConditionsRow {
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
                // array_mutator column.array == true
                let mut cell_bytes = row.get(12..12 + 16).unwrap();
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
            r#unknown28: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(28..28 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown32: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(32).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown33: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(33).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown34: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(34..34 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#unknown50: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(50).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown51: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(51).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown52: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(52..52 + 16).unwrap();
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
        })
        .collect()
});

#[derive(Debug)]
pub struct IndicatorConditionsRow {
    pub r#id: String,
    pub r#unknown8: i32,
    pub r#unknown12: Vec<StatsRef>,
    pub r#unknown28: i32,
    pub r#unknown32: bool,
    pub r#unknown33: bool,
    pub r#unknown34: StatsRef,
    pub r#unknown50: bool,
    pub r#unknown51: bool,
    pub r#unknown52: Vec<i32>,
    pub r#unknown68: i32,
    pub r#unknown72: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct IndicatorConditionsRef(pub usize);

impl Deref for IndicatorConditionsRef {
    type Target = IndicatorConditionsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_IndicatorConditions[self.0]
    }
}

impl IndicatorConditionsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static IndicatorConditionsRow {
        &TABLE_IndicatorConditions[self.0]
    }
    pub fn get(&self) -> &'static IndicatorConditionsRow {
        &TABLE_IndicatorConditions[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_IndicatorConditions
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static IndicatorConditionsRow)> {
        TABLE_IndicatorConditions
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
        for row in TABLE_IndicatorConditions.iter() {
            black_box(row);
        }
    }
}
