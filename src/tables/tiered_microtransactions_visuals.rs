#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_TieredMicrotransactionsVisuals: LazyLock<Vec<TieredMicrotransactionsVisualsRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/tieredmicrotransactionsvisuals.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| TieredMicrotransactionsVisualsRow {
                r#mtx: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(0..0 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    BaseItemTypesRef::new(value as usize)
                },
                r#tier: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(16..16 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#description: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(20..20 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#dds_file: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(28..28 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#unknown36: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(36..36 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown40: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(40..40 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct TieredMicrotransactionsVisualsRow {
    pub r#mtx: BaseItemTypesRef,
    pub r#tier: i32,
    pub r#description: String,
    pub r#dds_file: String,
    pub r#unknown36: i32,
    pub r#unknown40: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct TieredMicrotransactionsVisualsRef(pub usize);

impl Deref for TieredMicrotransactionsVisualsRef {
    type Target = TieredMicrotransactionsVisualsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_TieredMicrotransactionsVisuals[self.0]
    }
}

impl TieredMicrotransactionsVisualsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static TieredMicrotransactionsVisualsRow {
        &TABLE_TieredMicrotransactionsVisuals[self.0]
    }
    pub fn get(&self) -> &'static TieredMicrotransactionsVisualsRow {
        &TABLE_TieredMicrotransactionsVisuals[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_TieredMicrotransactionsVisuals
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs(
    ) -> impl Iterator<Item = (Self, &'static TieredMicrotransactionsVisualsRow)> {
        TABLE_TieredMicrotransactionsVisuals
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
        for row in TABLE_TieredMicrotransactionsVisuals.iter() {
            black_box(row);
        }
    }
}
