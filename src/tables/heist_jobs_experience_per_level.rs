#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_HeistJobsExperiencePerLevel: LazyLock<Vec<HeistJobsExperiencePerLevelRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/heistjobsexperienceperlevel.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| HeistJobsExperiencePerLevelRow {
            r#heist_jobs_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                HeistJobsRef::new(value as usize)
            },
            r#tier: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#experience: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#min_level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#achievement_items_key: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(28..28 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| AchievementItemsRef::new(value as usize)).collect()
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct HeistJobsExperiencePerLevelRow {
    pub r#heist_jobs_key: HeistJobsRef,
    pub r#tier: i32,
    pub r#experience: i32,
    pub r#min_level: i32,
    pub r#achievement_items_key: Vec<AchievementItemsRef>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct HeistJobsExperiencePerLevelRef(pub usize);

impl Deref for HeistJobsExperiencePerLevelRef {
    type Target = HeistJobsExperiencePerLevelRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_HeistJobsExperiencePerLevel[self.0]
    }
}

impl HeistJobsExperiencePerLevelRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static HeistJobsExperiencePerLevelRow {
        &TABLE_HeistJobsExperiencePerLevel[self.0]
    }
    pub fn get(&self) -> &'static HeistJobsExperiencePerLevelRow {
        &TABLE_HeistJobsExperiencePerLevel[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_HeistJobsExperiencePerLevel.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static HeistJobsExperiencePerLevelRow)> {
        TABLE_HeistJobsExperiencePerLevel.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_HeistJobsExperiencePerLevel.iter() {
            black_box(row);
        }
    }
}
