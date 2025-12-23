#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_AlternateTreeExtraPassiveOverridesFromJewelStats: LazyLock<Vec<AlternateTreeExtraPassiveOverridesFromJewelStatsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/alternatetreeextrapassiveoverridesfromjewelstats.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| AlternateTreeExtraPassiveOverridesFromJewelStatsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#radius_art: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                PassiveJewelRadiiArtRef::new(value as usize)
            },
            r#unknown24: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#passive_skill: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(28..28 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                PassiveSkillsRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct AlternateTreeExtraPassiveOverridesFromJewelStatsRow {
    pub r#id: String,
    pub r#radius_art: PassiveJewelRadiiArtRef,
    pub r#unknown24: i32,
    pub r#passive_skill: PassiveSkillsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct AlternateTreeExtraPassiveOverridesFromJewelStatsRef(pub usize);

impl Deref for AlternateTreeExtraPassiveOverridesFromJewelStatsRef {
    type Target = AlternateTreeExtraPassiveOverridesFromJewelStatsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_AlternateTreeExtraPassiveOverridesFromJewelStats[self.0]
    }
}

impl AlternateTreeExtraPassiveOverridesFromJewelStatsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static AlternateTreeExtraPassiveOverridesFromJewelStatsRow {
        &TABLE_AlternateTreeExtraPassiveOverridesFromJewelStats[self.0]
    }
    pub fn get(&self) -> &'static AlternateTreeExtraPassiveOverridesFromJewelStatsRow {
        &TABLE_AlternateTreeExtraPassiveOverridesFromJewelStats[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_AlternateTreeExtraPassiveOverridesFromJewelStats.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static AlternateTreeExtraPassiveOverridesFromJewelStatsRow)> {
        TABLE_AlternateTreeExtraPassiveOverridesFromJewelStats
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
        for row in TABLE_AlternateTreeExtraPassiveOverridesFromJewelStats.iter() {
            black_box(row);
        }
    }
}
