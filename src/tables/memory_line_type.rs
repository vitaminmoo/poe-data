#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MemoryLineType: LazyLock<Vec<MemoryLineTypeRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/memorylinetype.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| MemoryLineTypeRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#memory_line: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                AtlasMemoryLineRef::new(value as usize)
            },
            r#hash16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 2).unwrap();
                let value = cell_bytes.get_i16_le();
                value
            },
            r#stats: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(26..26 + 16).unwrap();
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
                let mut cell_bytes = row.get(42..42 + 16).unwrap();
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
            r#unknown58: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(58..58 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown62: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(62..62 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown66: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(66..66 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown82: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(82..82 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#suffix: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(86..86 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#mod: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(94..94 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ModsRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MemoryLineTypeRow {
    pub r#id: String,
    pub r#memory_line: AtlasMemoryLineRef,
    pub r#hash16: i16,
    pub r#stats: Vec<StatsRef>,
    pub r#stats_values: Vec<i32>,
    pub r#unknown58: i32,
    pub r#unknown62: i32,
    pub r#unknown66: i64,
    pub r#unknown82: i32,
    pub r#suffix: String,
    pub r#mod: ModsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MemoryLineTypeRef(pub usize);

impl Deref for MemoryLineTypeRef {
    type Target = MemoryLineTypeRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MemoryLineType[self.0]
    }
}

impl MemoryLineTypeRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MemoryLineTypeRow {
        &TABLE_MemoryLineType[self.0]
    }
    pub fn get(&self) -> &'static MemoryLineTypeRow {
        &TABLE_MemoryLineType[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MemoryLineType.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MemoryLineTypeRow)> {
        TABLE_MemoryLineType.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_MemoryLineType.iter() {
            black_box(row);
        }
    }
}
