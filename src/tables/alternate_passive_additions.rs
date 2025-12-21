#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_AlternatePassiveAdditions: LazyLock<Vec<AlternatePassiveAdditionsRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/alternatepassiveadditions.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| AlternatePassiveAdditionsRow {
                r#id: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(0..0 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#alternate_tree_version: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(8..8 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    AlternateTreeVersionsRef::new(value as usize)
                },
                r#spawn_weight: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(24..24 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#stats: {
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
                    values
                        .into_iter()
                        .map(|value| StatsRef::new(value as usize))
                        .collect()
                },
                r#stat1: {
                    // array_mutator column.array == false && column.type == 'interval'
                    let mut cell_bytes = row.get(44..44 + 8).unwrap();
                    let value = (cell_bytes.get_i32_le(), cell_bytes.get_i32_le());
                    value
                },
                r#stat2: {
                    // array_mutator column.array == false && column.type == 'interval'
                    let mut cell_bytes = row.get(52..52 + 8).unwrap();
                    let value = (cell_bytes.get_i32_le(), cell_bytes.get_i32_le());
                    value
                },
                r#stat3: {
                    // array_mutator column.array == false && column.type == 'interval'
                    let mut cell_bytes = row.get(60..60 + 8).unwrap();
                    let value = (cell_bytes.get_i32_le(), cell_bytes.get_i32_le());
                    value
                },
                r#passive_type: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(68..68 + 16).unwrap();
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
                r#unknown84: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(84..84 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct AlternatePassiveAdditionsRow {
    pub r#id: String,
    pub r#alternate_tree_version: AlternateTreeVersionsRef,
    pub r#spawn_weight: i32,
    pub r#stats: Vec<StatsRef>,
    pub r#stat1: (i32, i32),
    pub r#stat2: (i32, i32),
    pub r#stat3: (i32, i32),
    pub r#passive_type: Vec<i32>,
    pub r#unknown84: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct AlternatePassiveAdditionsRef(pub usize);

impl Deref for AlternatePassiveAdditionsRef {
    type Target = AlternatePassiveAdditionsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_AlternatePassiveAdditions[self.0]
    }
}

impl AlternatePassiveAdditionsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static AlternatePassiveAdditionsRow {
        &TABLE_AlternatePassiveAdditions[self.0]
    }
    pub fn get(&self) -> &'static AlternatePassiveAdditionsRow {
        &TABLE_AlternatePassiveAdditions[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_AlternatePassiveAdditions
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static AlternatePassiveAdditionsRow)> {
        TABLE_AlternatePassiveAdditions
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
        for row in TABLE_AlternatePassiveAdditions.iter() {
            black_box(row);
        }
    }
}
