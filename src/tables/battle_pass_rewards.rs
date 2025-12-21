#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_BattlePassRewards: LazyLock<Vec<BattlePassRewardsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/battlepassrewards.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| BattlePassRewardsRow {
            r#battle_pass: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BattlePassesRef::new(value as usize)
            },
            r#reward_tier: {
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
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(28).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(29..29 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#rewarded_mtx: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(37..37 + 16).unwrap();
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
                    .map(|value| BaseItemTypesRef::new(value as usize))
                    .collect()
            },
            r#unknown53: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(53..53 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#reward_description: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(57..57 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown65: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(65..65 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown73: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(73..73 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown77: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(77..77 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown81: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(81).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#reward_title: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(82..82 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown90: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(90..90 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown106: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(106).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown107: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(107..107 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown111: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(111..111 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown115: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(115..115 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown119: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(119..119 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown123: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(123..123 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown127: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(127..127 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown131: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(131..131 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown135: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(135..135 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown139: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(139).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown140: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(140).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown141: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(141).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct BattlePassRewardsRow {
    pub r#battle_pass: BattlePassesRef,
    pub r#reward_tier: i32,
    pub r#unknown20: i32,
    pub r#unknown24: i32,
    pub r#unknown28: bool,
    pub r#id: String,
    pub r#rewarded_mtx: Vec<BaseItemTypesRef>,
    pub r#unknown53: i32,
    pub r#reward_description: String,
    pub r#unknown65: String,
    pub r#unknown73: i32,
    pub r#unknown77: i32,
    pub r#unknown81: bool,
    pub r#reward_title: String,
    pub r#unknown90: i64,
    pub r#unknown106: bool,
    pub r#unknown107: i32,
    pub r#unknown111: i32,
    pub r#unknown115: i32,
    pub r#unknown119: i32,
    pub r#unknown123: i32,
    pub r#unknown127: i32,
    pub r#unknown131: i32,
    pub r#unknown135: i32,
    pub r#unknown139: bool,
    pub r#unknown140: bool,
    pub r#unknown141: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BattlePassRewardsRef(pub usize);

impl Deref for BattlePassRewardsRef {
    type Target = BattlePassRewardsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_BattlePassRewards[self.0]
    }
}

impl BattlePassRewardsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BattlePassRewardsRow {
        &TABLE_BattlePassRewards[self.0]
    }
    pub fn get(&self) -> &'static BattlePassRewardsRow {
        &TABLE_BattlePassRewards[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BattlePassRewards
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BattlePassRewardsRow)> {
        TABLE_BattlePassRewards
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
        for row in TABLE_BattlePassRewards.iter() {
            black_box(row);
        }
    }
}
