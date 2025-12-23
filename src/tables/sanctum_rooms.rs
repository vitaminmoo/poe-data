#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_SanctumRooms: LazyLock<Vec<SanctumRoomsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/sanctumrooms.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| SanctumRoomsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#arm_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#room_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                SanctumRoomTypesRef::new(value as usize)
            },
            r#script: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(32..32 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#floor: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                SanctumFloorsRef::new(value as usize)
            },
            r#area: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(56..56 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                WorldAreasRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct SanctumRoomsRow {
    pub r#id: String,
    pub r#arm_file: String,
    pub r#room_type: SanctumRoomTypesRef,
    pub r#script: String,
    pub r#floor: SanctumFloorsRef,
    pub r#area: WorldAreasRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct SanctumRoomsRef(pub usize);

impl Deref for SanctumRoomsRef {
    type Target = SanctumRoomsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_SanctumRooms[self.0]
    }
}

impl SanctumRoomsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static SanctumRoomsRow {
        &TABLE_SanctumRooms[self.0]
    }
    pub fn get(&self) -> &'static SanctumRoomsRow {
        &TABLE_SanctumRooms[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_SanctumRooms.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static SanctumRoomsRow)> {
        TABLE_SanctumRooms.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_SanctumRooms.iter() {
            black_box(row);
        }
    }
}
