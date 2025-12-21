#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_WarbandsPackNumbers: LazyLock<Vec<WarbandsPackNumbersRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/warbandspacknumbers.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| WarbandsPackNumbersRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#spawn_chance: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#min_level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(12..12 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#max_level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#tier4_number: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown24: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#tier3_number: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(28..28 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown32: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#tier2_number: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(36..36 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown40: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#tier1_number: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(44..44 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct WarbandsPackNumbersRow {
    pub r#id: String,
    pub r#spawn_chance: i32,
    pub r#min_level: i32,
    pub r#max_level: i32,
    pub r#tier4_number: i32,
    pub r#unknown24: i32,
    pub r#tier3_number: i32,
    pub r#unknown32: i32,
    pub r#tier2_number: i32,
    pub r#unknown40: i32,
    pub r#tier1_number: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct WarbandsPackNumbersRef(pub usize);

impl Deref for WarbandsPackNumbersRef {
    type Target = WarbandsPackNumbersRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_WarbandsPackNumbers[self.0]
    }
}

impl WarbandsPackNumbersRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static WarbandsPackNumbersRow {
        &TABLE_WarbandsPackNumbers[self.0]
    }
    pub fn get(&self) -> &'static WarbandsPackNumbersRow {
        &TABLE_WarbandsPackNumbers[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_WarbandsPackNumbers
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static WarbandsPackNumbersRow)> {
        TABLE_WarbandsPackNumbers
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
        for row in TABLE_WarbandsPackNumbers.iter() {
            black_box(row);
        }
    }
}
