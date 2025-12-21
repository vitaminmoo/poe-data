#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_HardModeExtraContentChances: LazyLock<Vec<HardModeExtraContentChancesRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/hardmodeextracontentchances.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| HardModeExtraContentChancesRow {
                r#id: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(0..0 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#unknown8: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(8..8 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown12: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(12..12 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown16: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(16..16 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown20: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(20..20 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown24: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(24..24 + 16).unwrap();
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
                r#unknown40: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(40..40 + 16).unwrap();
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
                r#unknown56: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(56..56 + 16).unwrap();
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
pub struct HardModeExtraContentChancesRow {
    pub r#id: String,
    pub r#unknown8: i32,
    pub r#unknown12: i32,
    pub r#unknown16: i32,
    pub r#unknown20: i32,
    pub r#unknown24: Vec<i64>,
    pub r#unknown40: Vec<i64>,
    pub r#unknown56: Vec<i64>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct HardModeExtraContentChancesRef(pub usize);

impl Deref for HardModeExtraContentChancesRef {
    type Target = HardModeExtraContentChancesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_HardModeExtraContentChances[self.0]
    }
}

impl HardModeExtraContentChancesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static HardModeExtraContentChancesRow {
        &TABLE_HardModeExtraContentChances[self.0]
    }
    pub fn get(&self) -> &'static HardModeExtraContentChancesRow {
        &TABLE_HardModeExtraContentChances[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_HardModeExtraContentChances
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static HardModeExtraContentChancesRow)>
    {
        TABLE_HardModeExtraContentChances
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
        for row in TABLE_HardModeExtraContentChances.iter() {
            black_box(row);
        }
    }
}
