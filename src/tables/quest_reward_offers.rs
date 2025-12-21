#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_QuestRewardOffers: LazyLock<Vec<QuestRewardOffersRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/questrewardoffers.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| QuestRewardOffersRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#quest_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                QuestRef::new(value as usize)
            },
            r#quest_flag: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                QuestFlagsRef::new(value as usize)
            },
            r#unknown40: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#reward_window_take: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(44..44 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ClientStringsRef::new(value as usize)
            },
            r#unknown60: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(60).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown61: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(61).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#reward_window_title: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(62..62 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ClientStringsRef::new(value as usize)
            },
            r#unknown78: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(78).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown79: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(79).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown80: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(80).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown81: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(81).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown82: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(82..82 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown98: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(98..98 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown114: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(114..114 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct QuestRewardOffersRow {
    pub r#id: String,
    pub r#quest_key: QuestRef,
    pub r#quest_flag: QuestFlagsRef,
    pub r#unknown40: i32,
    pub r#reward_window_take: ClientStringsRef,
    pub r#unknown60: bool,
    pub r#unknown61: bool,
    pub r#reward_window_title: ClientStringsRef,
    pub r#unknown78: bool,
    pub r#unknown79: bool,
    pub r#unknown80: bool,
    pub r#unknown81: bool,
    pub r#unknown82: i64,
    pub r#unknown98: i64,
    pub r#unknown114: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct QuestRewardOffersRef(pub usize);

impl Deref for QuestRewardOffersRef {
    type Target = QuestRewardOffersRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_QuestRewardOffers[self.0]
    }
}

impl QuestRewardOffersRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static QuestRewardOffersRow {
        &TABLE_QuestRewardOffers[self.0]
    }
    pub fn get(&self) -> &'static QuestRewardOffersRow {
        &TABLE_QuestRewardOffers[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_QuestRewardOffers
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static QuestRewardOffersRow)> {
        TABLE_QuestRewardOffers
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
        for row in TABLE_QuestRewardOffers.iter() {
            black_box(row);
        }
    }
}
