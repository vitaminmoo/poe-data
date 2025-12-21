#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_AdditionalMonsterPacksFromStats: LazyLock<
    Vec<AdditionalMonsterPacksFromStatsRow>,
> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/additionalmonsterpacksfromstats.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| AdditionalMonsterPacksFromStatsRow {
            r#stats_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#unknown16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#monster_packs_keys: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(20..20 + 16).unwrap();
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
                    .map(|value| MonsterPacksRef::new(value as usize))
                    .collect()
            },
            r#additional_monster_packs_stat_mode: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(36..36 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#pack_count_stats_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#stats_keys: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(56..56 + 16).unwrap();
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
            r#stats_values: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(72..72 + 16).unwrap();
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
            r#unknown88: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(88..88 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#pack_size_stats_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(92..92 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct AdditionalMonsterPacksFromStatsRow {
    pub r#stats_key: StatsRef,
    pub r#unknown16: i32,
    pub r#monster_packs_keys: Vec<MonsterPacksRef>,
    pub r#additional_monster_packs_stat_mode: i32,
    pub r#pack_count_stats_key: StatsRef,
    pub r#stats_keys: Vec<StatsRef>,
    pub r#stats_values: Vec<i32>,
    pub r#unknown88: i32,
    pub r#pack_size_stats_key: StatsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct AdditionalMonsterPacksFromStatsRef(pub usize);

impl Deref for AdditionalMonsterPacksFromStatsRef {
    type Target = AdditionalMonsterPacksFromStatsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_AdditionalMonsterPacksFromStats[self.0]
    }
}

impl AdditionalMonsterPacksFromStatsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static AdditionalMonsterPacksFromStatsRow {
        &TABLE_AdditionalMonsterPacksFromStats[self.0]
    }
    pub fn get(&self) -> &'static AdditionalMonsterPacksFromStatsRow {
        &TABLE_AdditionalMonsterPacksFromStats[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_AdditionalMonsterPacksFromStats
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs(
    ) -> impl Iterator<Item = (Self, &'static AdditionalMonsterPacksFromStatsRow)> {
        TABLE_AdditionalMonsterPacksFromStats
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
        for row in TABLE_AdditionalMonsterPacksFromStats.iter() {
            black_box(row);
        }
    }
}
