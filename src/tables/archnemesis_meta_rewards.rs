#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ArchnemesisMetaRewards: LazyLock<Vec<ArchnemesisMetaRewardsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/archnemesismetarewards.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ArchnemesisMetaRewardsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#reward_text: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#reward_group: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#script_argument: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(20..20 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#min_level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(28..28 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#max_level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ArchnemesisMetaRewardsRow {
    pub r#id: String,
    pub r#reward_text: String,
    pub r#reward_group: i32,
    pub r#script_argument: String,
    pub r#min_level: i32,
    pub r#max_level: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ArchnemesisMetaRewardsRef(pub usize);

impl Deref for ArchnemesisMetaRewardsRef {
    type Target = ArchnemesisMetaRewardsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ArchnemesisMetaRewards[self.0]
    }
}

impl ArchnemesisMetaRewardsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ArchnemesisMetaRewardsRow {
        &TABLE_ArchnemesisMetaRewards[self.0]
    }
    pub fn get(&self) -> &'static ArchnemesisMetaRewardsRow {
        &TABLE_ArchnemesisMetaRewards[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ArchnemesisMetaRewards.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ArchnemesisMetaRewardsRow)> {
        TABLE_ArchnemesisMetaRewards.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_ArchnemesisMetaRewards.iter() {
            black_box(row);
        }
    }
}
