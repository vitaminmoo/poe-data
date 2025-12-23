#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_SpawnAdditionalChestsOrClusters: LazyLock<Vec<SpawnAdditionalChestsOrClustersRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/spawnadditionalchestsorclusters.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| SpawnAdditionalChestsOrClustersRow {
            r#stats_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#chests_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ChestsRef::new(value as usize)
            },
            r#chest_clusters_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ChestClustersRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct SpawnAdditionalChestsOrClustersRow {
    pub r#stats_key: StatsRef,
    pub r#chests_key: ChestsRef,
    pub r#chest_clusters_key: ChestClustersRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct SpawnAdditionalChestsOrClustersRef(pub usize);

impl Deref for SpawnAdditionalChestsOrClustersRef {
    type Target = SpawnAdditionalChestsOrClustersRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_SpawnAdditionalChestsOrClusters[self.0]
    }
}

impl SpawnAdditionalChestsOrClustersRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static SpawnAdditionalChestsOrClustersRow {
        &TABLE_SpawnAdditionalChestsOrClusters[self.0]
    }
    pub fn get(&self) -> &'static SpawnAdditionalChestsOrClustersRow {
        &TABLE_SpawnAdditionalChestsOrClusters[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_SpawnAdditionalChestsOrClusters.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static SpawnAdditionalChestsOrClustersRow)> {
        TABLE_SpawnAdditionalChestsOrClusters.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_SpawnAdditionalChestsOrClusters.iter() {
            black_box(row);
        }
    }
}
