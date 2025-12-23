#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_SoulCoreStats: LazyLock<Vec<SoulCoreStatsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/soulcorestats.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| SoulCoreStatsRow {
            r#soul_core: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                SoulCoresRef::new(value as usize)
            },
            r#stat_category: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                SoulCoreStatCategoriesRef::new(value as usize)
            },
            r#stats: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
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
            r#stats_values: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(48..48 + 16).unwrap();
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
            r#bonded_stats: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(64..64 + 16).unwrap();
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
            r#bonded_stats_values: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(80..80 + 16).unwrap();
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
        })
        .collect()
});

#[derive(Debug)]
pub struct SoulCoreStatsRow {
    pub r#soul_core: SoulCoresRef,
    pub r#stat_category: SoulCoreStatCategoriesRef,
    pub r#stats: Vec<StatsRef>,
    pub r#stats_values: Vec<i32>,
    pub r#bonded_stats: Vec<StatsRef>,
    pub r#bonded_stats_values: Vec<i32>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct SoulCoreStatsRef(pub usize);

impl Deref for SoulCoreStatsRef {
    type Target = SoulCoreStatsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_SoulCoreStats[self.0]
    }
}

impl SoulCoreStatsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static SoulCoreStatsRow {
        &TABLE_SoulCoreStats[self.0]
    }
    pub fn get(&self) -> &'static SoulCoreStatsRow {
        &TABLE_SoulCoreStats[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_SoulCoreStats.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static SoulCoreStatsRow)> {
        TABLE_SoulCoreStats.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_SoulCoreStats.iter() {
            black_box(row);
        }
    }
}
