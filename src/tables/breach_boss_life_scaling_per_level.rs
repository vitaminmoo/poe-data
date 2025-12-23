#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_BreachBossLifeScalingPerLevel: LazyLock<Vec<BreachBossLifeScalingPerLevelRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/breachbosslifescalingperlevel.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| BreachBossLifeScalingPerLevelRow {
            r#monster_level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#life_multiplier: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(4..4 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct BreachBossLifeScalingPerLevelRow {
    pub r#monster_level: i32,
    pub r#life_multiplier: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BreachBossLifeScalingPerLevelRef(pub usize);

impl Deref for BreachBossLifeScalingPerLevelRef {
    type Target = BreachBossLifeScalingPerLevelRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_BreachBossLifeScalingPerLevel[self.0]
    }
}

impl BreachBossLifeScalingPerLevelRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BreachBossLifeScalingPerLevelRow {
        &TABLE_BreachBossLifeScalingPerLevel[self.0]
    }
    pub fn get(&self) -> &'static BreachBossLifeScalingPerLevelRow {
        &TABLE_BreachBossLifeScalingPerLevel[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BreachBossLifeScalingPerLevel.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BreachBossLifeScalingPerLevelRow)> {
        TABLE_BreachBossLifeScalingPerLevel.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_BreachBossLifeScalingPerLevel.iter() {
            black_box(row);
        }
    }
}
