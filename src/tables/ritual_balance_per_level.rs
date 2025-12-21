#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_RitualBalancePerLevel: LazyLock<Vec<RitualBalancePerLevelRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/ritualbalanceperlevel.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| RitualBalancePerLevelRow {
                r#min_level: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(0..0 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown4: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(4..4 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
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
                    let value = cell_bytes.get_f32_le();
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
            })
            .collect()
    });

#[derive(Debug)]
pub struct RitualBalancePerLevelRow {
    pub r#min_level: i32,
    pub r#unknown4: i32,
    pub r#unknown8: i32,
    pub r#unknown12: i32,
    pub r#unknown16: i32,
    pub r#unknown20: i32,
    pub r#unknown24: i32,
    pub r#unknown28: i32,
    pub r#unknown32: f32,
    pub r#unknown36: i32,
    pub r#unknown40: i32,
    pub r#unknown44: i32,
    pub r#unknown48: i32,
    pub r#unknown52: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct RitualBalancePerLevelRef(pub usize);

impl Deref for RitualBalancePerLevelRef {
    type Target = RitualBalancePerLevelRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_RitualBalancePerLevel[self.0]
    }
}

impl RitualBalancePerLevelRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static RitualBalancePerLevelRow {
        &TABLE_RitualBalancePerLevel[self.0]
    }
    pub fn get(&self) -> &'static RitualBalancePerLevelRow {
        &TABLE_RitualBalancePerLevel[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_RitualBalancePerLevel
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static RitualBalancePerLevelRow)> {
        TABLE_RitualBalancePerLevel
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
        for row in TABLE_RitualBalancePerLevel.iter() {
            black_box(row);
        }
    }
}
