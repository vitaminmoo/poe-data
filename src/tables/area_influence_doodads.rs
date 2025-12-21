#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_AreaInfluenceDoodads: LazyLock<Vec<AreaInfluenceDoodadsRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/areainfluencedoodads.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| AreaInfluenceDoodadsRow {
                r#stats_key: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(0..0 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
                r#stat_value: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(16..16 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown20: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(20..20 + 4).unwrap();
                    let value = cell_bytes.get_f32_le();
                    value
                },
                r#ao_files: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(24..24 + 16).unwrap();
                    let count = cell_bytes.get_u64_le() as usize;
                    let offset = cell_bytes.get_u64_le() as usize;
                    // array_mutator column.array == true && column.type == 'string'
                    let values = df.strings_from_offset(offset, count).unwrap();
                    values
                },
                r#unknown40: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(40..40 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown44: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(44).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#unknown45: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(45..45 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#unknown53: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(53..53 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct AreaInfluenceDoodadsRow {
    pub r#stats_key: StatsRef,
    pub r#stat_value: i32,
    pub r#unknown20: f32,
    pub r#ao_files: Vec<String>,
    pub r#unknown40: i32,
    pub r#unknown44: bool,
    pub r#unknown45: String,
    pub r#unknown53: StatsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct AreaInfluenceDoodadsRef(pub usize);

impl Deref for AreaInfluenceDoodadsRef {
    type Target = AreaInfluenceDoodadsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_AreaInfluenceDoodads[self.0]
    }
}

impl AreaInfluenceDoodadsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static AreaInfluenceDoodadsRow {
        &TABLE_AreaInfluenceDoodads[self.0]
    }
    pub fn get(&self) -> &'static AreaInfluenceDoodadsRow {
        &TABLE_AreaInfluenceDoodads[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_AreaInfluenceDoodads
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static AreaInfluenceDoodadsRow)> {
        TABLE_AreaInfluenceDoodads
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
        for row in TABLE_AreaInfluenceDoodads.iter() {
            black_box(row);
        }
    }
}
