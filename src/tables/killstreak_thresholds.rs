#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_KillstreakThresholds: LazyLock<Vec<KillstreakThresholdsRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/killstreakthresholds.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| KillstreakThresholdsRow {
                r#kills: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(0..0 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#monster_varieties_key: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(4..4 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    MonsterVarietiesRef::new(value as usize)
                },
                r#achievement_items_key: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(20..20 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    AchievementItemsRef::new(value as usize)
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct KillstreakThresholdsRow {
    pub r#kills: i32,
    pub r#monster_varieties_key: MonsterVarietiesRef,
    pub r#achievement_items_key: AchievementItemsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct KillstreakThresholdsRef(pub usize);

impl Deref for KillstreakThresholdsRef {
    type Target = KillstreakThresholdsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_KillstreakThresholds[self.0]
    }
}

impl KillstreakThresholdsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static KillstreakThresholdsRow {
        &TABLE_KillstreakThresholds[self.0]
    }
    pub fn get(&self) -> &'static KillstreakThresholdsRow {
        &TABLE_KillstreakThresholds[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_KillstreakThresholds
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static KillstreakThresholdsRow)> {
        TABLE_KillstreakThresholds
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
        for row in TABLE_KillstreakThresholds.iter() {
            black_box(row);
        }
    }
}
