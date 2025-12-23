#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_QuestStaticRewards: LazyLock<Vec<QuestStaticRewardsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/queststaticrewards.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| QuestStaticRewardsRow {
            r#quest_flag: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                QuestFlagsRef::new(value as usize)
            },
            r#unknown16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#reward_stats: {
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
                values.into_iter().map(|value| StatsRef::new(value as usize)).collect()
            },
            r#reward_stats_values: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(36..36 + 16).unwrap();
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
            r#quest_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(52..52 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                QuestRef::new(value as usize)
            },
            r#unknown68: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(68..68 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#message: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(72..72 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ClientStringsRef::new(value as usize)
            },
            r#unknown88: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(88..88 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown92: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(92).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct QuestStaticRewardsRow {
    pub r#quest_flag: QuestFlagsRef,
    pub r#unknown16: i32,
    pub r#reward_stats: Vec<StatsRef>,
    pub r#reward_stats_values: Vec<i32>,
    pub r#quest_key: QuestRef,
    pub r#unknown68: i32,
    pub r#message: ClientStringsRef,
    pub r#unknown88: i32,
    pub r#unknown92: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct QuestStaticRewardsRef(pub usize);

impl Deref for QuestStaticRewardsRef {
    type Target = QuestStaticRewardsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_QuestStaticRewards[self.0]
    }
}

impl QuestStaticRewardsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static QuestStaticRewardsRow {
        &TABLE_QuestStaticRewards[self.0]
    }
    pub fn get(&self) -> &'static QuestStaticRewardsRow {
        &TABLE_QuestStaticRewards[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_QuestStaticRewards.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static QuestStaticRewardsRow)> {
        TABLE_QuestStaticRewards.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_QuestStaticRewards.iter() {
            black_box(row);
        }
    }
}
