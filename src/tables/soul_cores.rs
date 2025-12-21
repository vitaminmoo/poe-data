#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_SoulCores: LazyLock<Vec<SoulCoresRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/soulcores.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| SoulCoresRow {
            r#base_item_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#required_level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#limit: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                SoulCoreLimitsRef::new(value as usize)
            },
            r#unknown36: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(36..36 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(52..52 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                SoulCoreTypesRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct SoulCoresRow {
    pub r#base_item_type: BaseItemTypesRef,
    pub r#required_level: i32,
    pub r#limit: SoulCoreLimitsRef,
    pub r#unknown36: i64,
    pub r#type: SoulCoreTypesRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct SoulCoresRef(pub usize);

impl Deref for SoulCoresRef {
    type Target = SoulCoresRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_SoulCores[self.0]
    }
}

impl SoulCoresRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static SoulCoresRow {
        &TABLE_SoulCores[self.0]
    }
    pub fn get(&self) -> &'static SoulCoresRow {
        &TABLE_SoulCores[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_SoulCores.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static SoulCoresRow)> {
        TABLE_SoulCores
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
        for row in TABLE_SoulCores.iter() {
            black_box(row);
        }
    }
}
