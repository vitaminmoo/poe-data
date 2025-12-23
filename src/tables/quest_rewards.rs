#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_QuestRewards: LazyLock<Vec<QuestRewardsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/questrewards.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| QuestRewardsRow {
            r#reward_offer: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                QuestRewardOffersRef::new(value as usize)
            },
            r#reward_order: {
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
                let value = cell_bytes.get_i32_le();
                value
            },
            r#reward: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(36..36 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#reward_level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(52..52 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#reward_rarity: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(56..56 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                RarityRef::new(value as usize)
            },
            r#unknown72: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(72..72 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown76: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(76..76 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown80: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(80..80 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown84: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(84..84 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown88: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(88..88 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown92: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(92..92 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown96: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(96..96 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#reward_stack: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(100..100 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown104: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(104).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown105: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(105).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown106: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(106..106 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown122: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(122..122 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown138: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(138..138 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown142: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(142..142 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown146: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(146..146 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown150: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(150..150 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown166: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(166..166 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown170: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(170..170 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown186: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(186..186 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown190: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(190).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct QuestRewardsRow {
    pub r#reward_offer: QuestRewardOffersRef,
    pub r#reward_order: i32,
    pub r#unknown20: i32,
    pub r#unknown24: i32,
    pub r#unknown28: i32,
    pub r#unknown32: i32,
    pub r#reward: BaseItemTypesRef,
    pub r#reward_level: i32,
    pub r#reward_rarity: RarityRef,
    pub r#unknown72: i32,
    pub r#unknown76: i32,
    pub r#unknown80: i32,
    pub r#unknown84: i32,
    pub r#unknown88: i32,
    pub r#unknown92: i32,
    pub r#unknown96: i32,
    pub r#reward_stack: i32,
    pub r#unknown104: bool,
    pub r#unknown105: bool,
    pub r#unknown106: (usize, usize),
    pub r#unknown122: i64,
    pub r#unknown138: i32,
    pub r#unknown142: i32,
    pub r#unknown146: i32,
    pub r#unknown150: i64,
    pub r#unknown166: i32,
    pub r#unknown170: (usize, usize),
    pub r#unknown186: i32,
    pub r#unknown190: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct QuestRewardsRef(pub usize);

impl Deref for QuestRewardsRef {
    type Target = QuestRewardsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_QuestRewards[self.0]
    }
}

impl QuestRewardsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static QuestRewardsRow {
        &TABLE_QuestRewards[self.0]
    }
    pub fn get(&self) -> &'static QuestRewardsRow {
        &TABLE_QuestRewards[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_QuestRewards.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static QuestRewardsRow)> {
        TABLE_QuestRewards.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_QuestRewards.iter() {
            black_box(row);
        }
    }
}
