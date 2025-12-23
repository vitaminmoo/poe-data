#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_DelveRewardTierConstants: LazyLock<Vec<DelveRewardTierConstantsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/delverewardtierconstants.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| DelveRewardTierConstantsRow {
            r#unknown0: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown4: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(4..4 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown8: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown12: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(12..12 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct DelveRewardTierConstantsRow {
    pub r#unknown0: i32,
    pub r#unknown4: i32,
    pub r#unknown8: i32,
    pub r#unknown12: i32,
    pub r#unknown16: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct DelveRewardTierConstantsRef(pub usize);

impl Deref for DelveRewardTierConstantsRef {
    type Target = DelveRewardTierConstantsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_DelveRewardTierConstants[self.0]
    }
}

impl DelveRewardTierConstantsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static DelveRewardTierConstantsRow {
        &TABLE_DelveRewardTierConstants[self.0]
    }
    pub fn get(&self) -> &'static DelveRewardTierConstantsRow {
        &TABLE_DelveRewardTierConstants[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_DelveRewardTierConstants.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static DelveRewardTierConstantsRow)> {
        TABLE_DelveRewardTierConstants.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_DelveRewardTierConstants.iter() {
            black_box(row);
        }
    }
}
