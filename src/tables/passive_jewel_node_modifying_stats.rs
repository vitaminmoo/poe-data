#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_PassiveJewelNodeModifyingStats: LazyLock<Vec<PassiveJewelNodeModifyingStatsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/passivejewelnodemodifyingstats.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| PassiveJewelNodeModifyingStatsRow {
            r#jwel_stat: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#stat: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
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
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(34).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown35: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(35).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct PassiveJewelNodeModifyingStatsRow {
    pub r#jwel_stat: StatsRef,
    pub r#stat: StatsRef,
    pub r#unknown32: bool,
    pub r#unknown33: bool,
    pub r#unknown34: bool,
    pub r#unknown35: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct PassiveJewelNodeModifyingStatsRef(pub usize);

impl Deref for PassiveJewelNodeModifyingStatsRef {
    type Target = PassiveJewelNodeModifyingStatsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_PassiveJewelNodeModifyingStats[self.0]
    }
}

impl PassiveJewelNodeModifyingStatsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static PassiveJewelNodeModifyingStatsRow {
        &TABLE_PassiveJewelNodeModifyingStats[self.0]
    }
    pub fn get(&self) -> &'static PassiveJewelNodeModifyingStatsRow {
        &TABLE_PassiveJewelNodeModifyingStats[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_PassiveJewelNodeModifyingStats.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static PassiveJewelNodeModifyingStatsRow)> {
        TABLE_PassiveJewelNodeModifyingStats.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_PassiveJewelNodeModifyingStats.iter() {
            black_box(row);
        }
    }
}
