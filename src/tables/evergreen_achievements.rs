#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_EvergreenAchievements: LazyLock<Vec<EvergreenAchievementsRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/evergreenachievements.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| EvergreenAchievementsRow {
                r#unknown0: {
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
                r#unknown8: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(8..8 + 16).unwrap();
                    let count = cell_bytes.get_u64_le() as usize;
                    let offset = cell_bytes.get_u64_le() as usize;
                    // array_mutator column.array == true && column.type else
                    let values = df
                        .array_from_offset(offset, count, 16)
                        .unwrap()
                        .iter()
                        .map(|x| x.clone().get_i64_le())
                        .collect::<Vec<i64>>();
                    values
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct EvergreenAchievementsRow {
    pub r#unknown0: i32,
    pub r#unknown4: i32,
    pub r#unknown8: Vec<i64>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct EvergreenAchievementsRef(pub usize);

impl Deref for EvergreenAchievementsRef {
    type Target = EvergreenAchievementsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_EvergreenAchievements[self.0]
    }
}

impl EvergreenAchievementsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static EvergreenAchievementsRow {
        &TABLE_EvergreenAchievements[self.0]
    }
    pub fn get(&self) -> &'static EvergreenAchievementsRow {
        &TABLE_EvergreenAchievements[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_EvergreenAchievements
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static EvergreenAchievementsRow)> {
        TABLE_EvergreenAchievements
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
        for row in TABLE_EvergreenAchievements.iter() {
            black_box(row);
        }
    }
}
