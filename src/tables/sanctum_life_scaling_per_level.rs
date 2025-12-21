#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_SanctumLifeScalingPerLevel: LazyLock<Vec<SanctumLifeScalingPerLevelRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/sanctumlifescalingperlevel.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| SanctumLifeScalingPerLevelRow {
                r#level: {
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
            })
            .collect()
    });

#[derive(Debug)]
pub struct SanctumLifeScalingPerLevelRow {
    pub r#level: i32,
    pub r#unknown4: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct SanctumLifeScalingPerLevelRef(pub usize);

impl Deref for SanctumLifeScalingPerLevelRef {
    type Target = SanctumLifeScalingPerLevelRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_SanctumLifeScalingPerLevel[self.0]
    }
}

impl SanctumLifeScalingPerLevelRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static SanctumLifeScalingPerLevelRow {
        &TABLE_SanctumLifeScalingPerLevel[self.0]
    }
    pub fn get(&self) -> &'static SanctumLifeScalingPerLevelRow {
        &TABLE_SanctumLifeScalingPerLevel[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_SanctumLifeScalingPerLevel
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static SanctumLifeScalingPerLevelRow)>
    {
        TABLE_SanctumLifeScalingPerLevel
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
        for row in TABLE_SanctumLifeScalingPerLevel.iter() {
            black_box(row);
        }
    }
}
