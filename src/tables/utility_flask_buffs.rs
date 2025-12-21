#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_UtilityFlaskBuffs: LazyLock<Vec<UtilityFlaskBuffsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/utilityflaskbuffs.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| UtilityFlaskBuffsRow {
            r#buff_definition: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BuffDefinitionsRef::new(value as usize)
            },
            r#stat_values: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
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
            r#stat_values2: {
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
            r#unknown48: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(48..48 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct UtilityFlaskBuffsRow {
    pub r#buff_definition: BuffDefinitionsRef,
    pub r#stat_values: Vec<i32>,
    pub r#stat_values2: Vec<i32>,
    pub r#unknown48: i64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct UtilityFlaskBuffsRef(pub usize);

impl Deref for UtilityFlaskBuffsRef {
    type Target = UtilityFlaskBuffsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_UtilityFlaskBuffs[self.0]
    }
}

impl UtilityFlaskBuffsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static UtilityFlaskBuffsRow {
        &TABLE_UtilityFlaskBuffs[self.0]
    }
    pub fn get(&self) -> &'static UtilityFlaskBuffsRow {
        &TABLE_UtilityFlaskBuffs[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_UtilityFlaskBuffs
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static UtilityFlaskBuffsRow)> {
        TABLE_UtilityFlaskBuffs
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
        for row in TABLE_UtilityFlaskBuffs.iter() {
            black_box(row);
        }
    }
}
