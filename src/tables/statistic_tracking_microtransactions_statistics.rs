#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_StatisticTrackingMicrotransactionsStatistics: LazyLock<
    Vec<StatisticTrackingMicrotransactionsStatisticsRow>,
> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/statistictrackingmicrotransactionsstatistics.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| StatisticTrackingMicrotransactionsStatisticsRow {
            r#statistic_tracking_mtx: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatisticTrackingMicrotransactionsRef::new(value as usize)
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
            r#icon: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(41..41 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown49: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(49).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown50: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(50..50 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#counter_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(66..66 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatisticTrackingMicrotransactionCounterTypeRef::new(value as usize)
            },
            r#unknown82: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(82..82 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct StatisticTrackingMicrotransactionsStatisticsRow {
    pub r#statistic_tracking_mtx: StatisticTrackingMicrotransactionsRef,
    pub r#unknown16: i32,
    pub r#unknown20: i32,
    pub r#unknown24: i64,
    pub r#unknown40: bool,
    pub r#icon: String,
    pub r#unknown49: bool,
    pub r#unknown50: i64,
    pub r#counter_type: StatisticTrackingMicrotransactionCounterTypeRef,
    pub r#unknown82: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct StatisticTrackingMicrotransactionsStatisticsRef(pub usize);

impl Deref for StatisticTrackingMicrotransactionsStatisticsRef {
    type Target = StatisticTrackingMicrotransactionsStatisticsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_StatisticTrackingMicrotransactionsStatistics[self.0]
    }
}

impl StatisticTrackingMicrotransactionsStatisticsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static StatisticTrackingMicrotransactionsStatisticsRow {
        &TABLE_StatisticTrackingMicrotransactionsStatistics[self.0]
    }
    pub fn get(&self) -> &'static StatisticTrackingMicrotransactionsStatisticsRow {
        &TABLE_StatisticTrackingMicrotransactionsStatistics[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_StatisticTrackingMicrotransactionsStatistics
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<
        Item = (
            Self,
            &'static StatisticTrackingMicrotransactionsStatisticsRow,
        ),
    > {
        TABLE_StatisticTrackingMicrotransactionsStatistics
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
        for row in TABLE_StatisticTrackingMicrotransactionsStatistics.iter() {
            black_box(row);
        }
    }
}
