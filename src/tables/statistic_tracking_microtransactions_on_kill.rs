#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_StatisticTrackingMicrotransactionsOnKill: LazyLock<Vec<StatisticTrackingMicrotransactionsOnKillRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/statistictrackingmicrotransactionsonkill.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| StatisticTrackingMicrotransactionsOnKillRow {
            r#unknown0: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct StatisticTrackingMicrotransactionsOnKillRow {
    pub r#unknown0: i64,
    pub r#unknown16: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct StatisticTrackingMicrotransactionsOnKillRef(pub usize);

impl Deref for StatisticTrackingMicrotransactionsOnKillRef {
    type Target = StatisticTrackingMicrotransactionsOnKillRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_StatisticTrackingMicrotransactionsOnKill[self.0]
    }
}

impl StatisticTrackingMicrotransactionsOnKillRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static StatisticTrackingMicrotransactionsOnKillRow {
        &TABLE_StatisticTrackingMicrotransactionsOnKill[self.0]
    }
    pub fn get(&self) -> &'static StatisticTrackingMicrotransactionsOnKillRow {
        &TABLE_StatisticTrackingMicrotransactionsOnKill[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_StatisticTrackingMicrotransactionsOnKill.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static StatisticTrackingMicrotransactionsOnKillRow)> {
        TABLE_StatisticTrackingMicrotransactionsOnKill.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_StatisticTrackingMicrotransactionsOnKill.iter() {
            black_box(row);
        }
    }
}
