#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_BetrayalTraitorRewards: LazyLock<Vec<BetrayalTraitorRewardsRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/betrayaltraitorrewards.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| BetrayalTraitorRewardsRow {
                r#betrayal_jobs_key: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(0..0 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    BetrayalJobsRef::new(value as usize)
                },
                r#betrayal_targets_key: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(16..16 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    BetrayalTargetsRef::new(value as usize)
                },
                r#betrayal_ranks_key: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(32..32 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    BetrayalRanksRef::new(value as usize)
                },
                r#description: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(48..48 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#description2: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(56..56 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct BetrayalTraitorRewardsRow {
    pub r#betrayal_jobs_key: BetrayalJobsRef,
    pub r#betrayal_targets_key: BetrayalTargetsRef,
    pub r#betrayal_ranks_key: BetrayalRanksRef,
    pub r#description: String,
    pub r#description2: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BetrayalTraitorRewardsRef(pub usize);

impl Deref for BetrayalTraitorRewardsRef {
    type Target = BetrayalTraitorRewardsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_BetrayalTraitorRewards[self.0]
    }
}

impl BetrayalTraitorRewardsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BetrayalTraitorRewardsRow {
        &TABLE_BetrayalTraitorRewards[self.0]
    }
    pub fn get(&self) -> &'static BetrayalTraitorRewardsRow {
        &TABLE_BetrayalTraitorRewards[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BetrayalTraitorRewards
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BetrayalTraitorRewardsRow)> {
        TABLE_BetrayalTraitorRewards
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
        for row in TABLE_BetrayalTraitorRewards.iter() {
            black_box(row);
        }
    }
}
