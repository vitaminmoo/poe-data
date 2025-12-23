#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_AlternateQualityTypes: LazyLock<Vec<AlternateQualityTypesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/alternatequalitytypes.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| AlternateQualityTypesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#description: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#item: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#unknown32: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#hash16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(48..48 + 2).unwrap();
                let value = cell_bytes.get_u16_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct AlternateQualityTypesRow {
    pub r#id: String,
    pub r#description: String,
    pub r#item: BaseItemTypesRef,
    pub r#unknown32: i64,
    pub r#hash16: u16,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct AlternateQualityTypesRef(pub usize);

impl Deref for AlternateQualityTypesRef {
    type Target = AlternateQualityTypesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_AlternateQualityTypes[self.0]
    }
}

impl AlternateQualityTypesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static AlternateQualityTypesRow {
        &TABLE_AlternateQualityTypes[self.0]
    }
    pub fn get(&self) -> &'static AlternateQualityTypesRow {
        &TABLE_AlternateQualityTypes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_AlternateQualityTypes.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static AlternateQualityTypesRow)> {
        TABLE_AlternateQualityTypes.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_AlternateQualityTypes.iter() {
            black_box(row);
        }
    }
}
