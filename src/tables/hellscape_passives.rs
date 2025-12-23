#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_HellscapePassives: LazyLock<Vec<HellscapePassivesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/hellscapepassives.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| HellscapePassivesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#stats: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| StatsRef::new(value as usize)).collect()
            },
            r#stats_values: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 4)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i32_le())
                    .collect::<Vec<i32>>();
                values
            },
            r#points: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(48..48 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#hash16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(52..52 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#icon: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(56..56 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#icon_maxed: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(64..64 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#sound_effect: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(72..72 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                SoundEffectsRef::new(value as usize)
            },
            r#unknown88: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(88..88 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#quest: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(92..92 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                QuestRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct HellscapePassivesRow {
    pub r#id: String,
    pub r#name: String,
    pub r#stats: Vec<StatsRef>,
    pub r#stats_values: Vec<i32>,
    pub r#points: i32,
    pub r#hash16: i32,
    pub r#icon: String,
    pub r#icon_maxed: String,
    pub r#sound_effect: SoundEffectsRef,
    pub r#unknown88: i32,
    pub r#quest: QuestRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct HellscapePassivesRef(pub usize);

impl Deref for HellscapePassivesRef {
    type Target = HellscapePassivesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_HellscapePassives[self.0]
    }
}

impl HellscapePassivesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static HellscapePassivesRow {
        &TABLE_HellscapePassives[self.0]
    }
    pub fn get(&self) -> &'static HellscapePassivesRow {
        &TABLE_HellscapePassives[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_HellscapePassives.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static HellscapePassivesRow)> {
        TABLE_HellscapePassives.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_HellscapePassives.iter() {
            black_box(row);
        }
    }
}
