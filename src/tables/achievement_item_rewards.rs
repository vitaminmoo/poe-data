#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_AchievementItemRewards: LazyLock<Vec<AchievementItemRewardsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/achievementitemrewards.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| AchievementItemRewardsRow {
            r#achievement_item: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                AchievementItemsRef::new(value as usize)
            },
            r#base_item_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#message: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(32..32 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(40..40 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown48: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(48..48 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct AchievementItemRewardsRow {
    pub r#achievement_item: AchievementItemsRef,
    pub r#base_item_type: BaseItemTypesRef,
    pub r#message: String,
    pub r#id: String,
    pub r#unknown48: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct AchievementItemRewardsRef(pub usize);

impl Deref for AchievementItemRewardsRef {
    type Target = AchievementItemRewardsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_AchievementItemRewards[self.0]
    }
}

impl AchievementItemRewardsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static AchievementItemRewardsRow {
        &TABLE_AchievementItemRewards[self.0]
    }
    pub fn get(&self) -> &'static AchievementItemRewardsRow {
        &TABLE_AchievementItemRewards[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_AchievementItemRewards.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static AchievementItemRewardsRow)> {
        TABLE_AchievementItemRewards.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_AchievementItemRewards.iter() {
            black_box(row);
        }
    }
}
