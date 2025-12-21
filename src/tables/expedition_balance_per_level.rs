#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ExpeditionBalancePerLevel: LazyLock<Vec<ExpeditionBalancePerLevelRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/expeditionbalanceperlevel.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| ExpeditionBalancePerLevelRow {
                r#level: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(0..0 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown4: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(4).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#unknown5: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(5..5 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown9: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(9..9 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown13: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(13..13 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown17: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(17..17 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown21: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(21..21 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown25: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(25..25 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown29: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(29..29 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown33: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(33..33 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown37: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(37..37 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown41: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(41..41 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown45: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(45..45 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown49: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(49..49 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown53: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(53..53 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct ExpeditionBalancePerLevelRow {
    pub r#level: i32,
    pub r#unknown4: bool,
    pub r#unknown5: i32,
    pub r#unknown9: i32,
    pub r#unknown13: i32,
    pub r#unknown17: i32,
    pub r#unknown21: i32,
    pub r#unknown25: i32,
    pub r#unknown29: i32,
    pub r#unknown33: i32,
    pub r#unknown37: i32,
    pub r#unknown41: i32,
    pub r#unknown45: i32,
    pub r#unknown49: i32,
    pub r#unknown53: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ExpeditionBalancePerLevelRef(pub usize);

impl Deref for ExpeditionBalancePerLevelRef {
    type Target = ExpeditionBalancePerLevelRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ExpeditionBalancePerLevel[self.0]
    }
}

impl ExpeditionBalancePerLevelRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ExpeditionBalancePerLevelRow {
        &TABLE_ExpeditionBalancePerLevel[self.0]
    }
    pub fn get(&self) -> &'static ExpeditionBalancePerLevelRow {
        &TABLE_ExpeditionBalancePerLevel[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ExpeditionBalancePerLevel
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ExpeditionBalancePerLevelRow)> {
        TABLE_ExpeditionBalancePerLevel
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
        for row in TABLE_ExpeditionBalancePerLevel.iter() {
            black_box(row);
        }
    }
}
