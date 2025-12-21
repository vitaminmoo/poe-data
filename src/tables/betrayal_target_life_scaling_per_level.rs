#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_BetrayalTargetLifeScalingPerLevel: LazyLock<
    Vec<BetrayalTargetLifeScalingPerLevelRow>,
> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/betrayaltargetlifescalingperlevel.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| BetrayalTargetLifeScalingPerLevelRow {
            r#level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#more_life: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(4..4 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct BetrayalTargetLifeScalingPerLevelRow {
    pub r#level: i32,
    pub r#more_life: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BetrayalTargetLifeScalingPerLevelRef(pub usize);

impl Deref for BetrayalTargetLifeScalingPerLevelRef {
    type Target = BetrayalTargetLifeScalingPerLevelRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_BetrayalTargetLifeScalingPerLevel[self.0]
    }
}

impl BetrayalTargetLifeScalingPerLevelRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BetrayalTargetLifeScalingPerLevelRow {
        &TABLE_BetrayalTargetLifeScalingPerLevel[self.0]
    }
    pub fn get(&self) -> &'static BetrayalTargetLifeScalingPerLevelRow {
        &TABLE_BetrayalTargetLifeScalingPerLevel[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BetrayalTargetLifeScalingPerLevel
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs(
    ) -> impl Iterator<Item = (Self, &'static BetrayalTargetLifeScalingPerLevelRow)> {
        TABLE_BetrayalTargetLifeScalingPerLevel
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
        for row in TABLE_BetrayalTargetLifeScalingPerLevel.iter() {
            black_box(row);
        }
    }
}
