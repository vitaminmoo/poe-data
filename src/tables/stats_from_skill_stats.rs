#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_StatsFromSkillStats: LazyLock<Vec<StatsFromSkillStatsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/statsfromskillstats.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| StatsFromSkillStatsRow {
            r#skill_condition: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#granted_flag: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#flag_value: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(32).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct StatsFromSkillStatsRow {
    pub r#skill_condition: StatsRef,
    pub r#granted_flag: StatsRef,
    pub r#flag_value: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct StatsFromSkillStatsRef(pub usize);

impl Deref for StatsFromSkillStatsRef {
    type Target = StatsFromSkillStatsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_StatsFromSkillStats[self.0]
    }
}

impl StatsFromSkillStatsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static StatsFromSkillStatsRow {
        &TABLE_StatsFromSkillStats[self.0]
    }
    pub fn get(&self) -> &'static StatsFromSkillStatsRow {
        &TABLE_StatsFromSkillStats[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_StatsFromSkillStats.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static StatsFromSkillStatsRow)> {
        TABLE_StatsFromSkillStats.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_StatsFromSkillStats.iter() {
            black_box(row);
        }
    }
}
