#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MinionGemLevelScaling: LazyLock<Vec<MinionGemLevelScalingRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/miniongemlevelscaling.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| MinionGemLevelScalingRow {
                r#gem_level: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(0..0 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#minion_level: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(4..4 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct MinionGemLevelScalingRow {
    pub r#gem_level: i32,
    pub r#minion_level: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MinionGemLevelScalingRef(pub usize);

impl Deref for MinionGemLevelScalingRef {
    type Target = MinionGemLevelScalingRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MinionGemLevelScaling[self.0]
    }
}

impl MinionGemLevelScalingRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MinionGemLevelScalingRow {
        &TABLE_MinionGemLevelScaling[self.0]
    }
    pub fn get(&self) -> &'static MinionGemLevelScalingRow {
        &TABLE_MinionGemLevelScaling[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MinionGemLevelScaling
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MinionGemLevelScalingRow)> {
        TABLE_MinionGemLevelScaling
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
        for row in TABLE_MinionGemLevelScaling.iter() {
            black_box(row);
        }
    }
}
