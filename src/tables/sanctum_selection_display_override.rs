#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_SanctumSelectionDisplayOverride: LazyLock<Vec<SanctumSelectionDisplayOverrideRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/sanctumselectiondisplayoverride.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| SanctumSelectionDisplayOverrideRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#reward: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#icon: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#downside: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(24..24 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#item: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct SanctumSelectionDisplayOverrideRow {
    pub r#id: String,
    pub r#reward: String,
    pub r#icon: String,
    pub r#downside: String,
    pub r#item: BaseItemTypesRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct SanctumSelectionDisplayOverrideRef(pub usize);

impl Deref for SanctumSelectionDisplayOverrideRef {
    type Target = SanctumSelectionDisplayOverrideRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_SanctumSelectionDisplayOverride[self.0]
    }
}

impl SanctumSelectionDisplayOverrideRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static SanctumSelectionDisplayOverrideRow {
        &TABLE_SanctumSelectionDisplayOverride[self.0]
    }
    pub fn get(&self) -> &'static SanctumSelectionDisplayOverrideRow {
        &TABLE_SanctumSelectionDisplayOverride[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_SanctumSelectionDisplayOverride.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static SanctumSelectionDisplayOverrideRow)> {
        TABLE_SanctumSelectionDisplayOverride.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_SanctumSelectionDisplayOverride.iter() {
            black_box(row);
        }
    }
}
