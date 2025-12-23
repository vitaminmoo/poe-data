#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_SanctumFloors: LazyLock<Vec<SanctumFloorsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/sanctumfloors.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| SanctumFloorsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#area: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                WorldAreasRef::new(value as usize)
            },
            r#title: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ClientStringsRef::new(value as usize)
            },
            r#room_icon: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(40..40 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#boss_icon: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(48..48 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#description: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(56..56 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#summary: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(64..64 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ClientStringsRef::new(value as usize)
            },
            r#itemised: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(80..80 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct SanctumFloorsRow {
    pub r#id: String,
    pub r#area: WorldAreasRef,
    pub r#title: ClientStringsRef,
    pub r#room_icon: String,
    pub r#boss_icon: String,
    pub r#description: String,
    pub r#summary: ClientStringsRef,
    pub r#itemised: BaseItemTypesRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct SanctumFloorsRef(pub usize);

impl Deref for SanctumFloorsRef {
    type Target = SanctumFloorsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_SanctumFloors[self.0]
    }
}

impl SanctumFloorsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static SanctumFloorsRow {
        &TABLE_SanctumFloors[self.0]
    }
    pub fn get(&self) -> &'static SanctumFloorsRow {
        &TABLE_SanctumFloors[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_SanctumFloors.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static SanctumFloorsRow)> {
        TABLE_SanctumFloors.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_SanctumFloors.iter() {
            black_box(row);
        }
    }
}
