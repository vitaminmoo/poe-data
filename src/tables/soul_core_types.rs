#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_SoulCoreTypes: LazyLock<Vec<SoulCoreTypesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/soulcoretypes.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| SoulCoreTypesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#effect_stat: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(24..24 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#socketed_stat: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct SoulCoreTypesRow {
    pub r#id: String,
    pub r#effect_stat: StatsRef,
    pub r#name: String,
    pub r#socketed_stat: StatsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct SoulCoreTypesRef(pub usize);

impl Deref for SoulCoreTypesRef {
    type Target = SoulCoreTypesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_SoulCoreTypes[self.0]
    }
}

impl SoulCoreTypesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static SoulCoreTypesRow {
        &TABLE_SoulCoreTypes[self.0]
    }
    pub fn get(&self) -> &'static SoulCoreTypesRow {
        &TABLE_SoulCoreTypes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_SoulCoreTypes.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static SoulCoreTypesRow)> {
        TABLE_SoulCoreTypes
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
        for row in TABLE_SoulCoreTypes.iter() {
            black_box(row);
        }
    }
}
