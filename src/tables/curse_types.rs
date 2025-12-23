#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_CurseTypes: LazyLock<Vec<CurseTypesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/cursetypes.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| CurseTypesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown8: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown24: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(24).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#hexproof_stat: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(25..25 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#unknown41: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(41..41 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct CurseTypesRow {
    pub r#id: String,
    pub r#unknown8: i64,
    pub r#unknown24: bool,
    pub r#hexproof_stat: StatsRef,
    pub r#unknown41: i64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct CurseTypesRef(pub usize);

impl Deref for CurseTypesRef {
    type Target = CurseTypesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_CurseTypes[self.0]
    }
}

impl CurseTypesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static CurseTypesRow {
        &TABLE_CurseTypes[self.0]
    }
    pub fn get(&self) -> &'static CurseTypesRow {
        &TABLE_CurseTypes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_CurseTypes.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static CurseTypesRow)> {
        TABLE_CurseTypes.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_CurseTypes.iter() {
            black_box(row);
        }
    }
}
