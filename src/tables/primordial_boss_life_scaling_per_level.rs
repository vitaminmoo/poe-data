#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_PrimordialBossLifeScalingPerLevel: LazyLock<Vec<PrimordialBossLifeScalingPerLevelRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/primordialbosslifescalingperlevel.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| PrimordialBossLifeScalingPerLevelRow {
            r#area_level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#scale: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(4..4 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct PrimordialBossLifeScalingPerLevelRow {
    pub r#area_level: i32,
    pub r#scale: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct PrimordialBossLifeScalingPerLevelRef(pub usize);

impl Deref for PrimordialBossLifeScalingPerLevelRef {
    type Target = PrimordialBossLifeScalingPerLevelRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_PrimordialBossLifeScalingPerLevel[self.0]
    }
}

impl PrimordialBossLifeScalingPerLevelRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static PrimordialBossLifeScalingPerLevelRow {
        &TABLE_PrimordialBossLifeScalingPerLevel[self.0]
    }
    pub fn get(&self) -> &'static PrimordialBossLifeScalingPerLevelRow {
        &TABLE_PrimordialBossLifeScalingPerLevel[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_PrimordialBossLifeScalingPerLevel.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static PrimordialBossLifeScalingPerLevelRow)> {
        TABLE_PrimordialBossLifeScalingPerLevel.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_PrimordialBossLifeScalingPerLevel.iter() {
            black_box(row);
        }
    }
}
