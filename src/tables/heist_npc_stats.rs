#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_HeistNPCStats: LazyLock<Vec<HeistNPCStatsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/heistnpcstats.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| HeistNPCStatsRow {
            r#stats_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#unknown16: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(16).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown17: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(17).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown18: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(18).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown19: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(19).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct HeistNPCStatsRow {
    pub r#stats_key: StatsRef,
    pub r#unknown16: bool,
    pub r#unknown17: bool,
    pub r#unknown18: bool,
    pub r#unknown19: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct HeistNPCStatsRef(pub usize);

impl Deref for HeistNPCStatsRef {
    type Target = HeistNPCStatsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_HeistNPCStats[self.0]
    }
}

impl HeistNPCStatsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static HeistNPCStatsRow {
        &TABLE_HeistNPCStats[self.0]
    }
    pub fn get(&self) -> &'static HeistNPCStatsRow {
        &TABLE_HeistNPCStats[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_HeistNPCStats.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static HeistNPCStatsRow)> {
        TABLE_HeistNPCStats
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
        for row in TABLE_HeistNPCStats.iter() {
            black_box(row);
        }
    }
}
