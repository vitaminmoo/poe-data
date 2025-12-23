#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_SanctumRoomTypes: LazyLock<Vec<SanctumRoomTypesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/sanctumroomtypes.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| SanctumRoomTypesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown8: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(8).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown9: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(9).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown10: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(10..10 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown26: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(26..26 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown42: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(42).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#icon: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(43..43 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown51: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(51).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#description: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(52..52 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown60: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(60..60 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#rooms: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(76..76 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| SanctumRoomsRef::new(value as usize)).collect()
            },
            r#unknown92: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(92..92 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown100: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(100).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct SanctumRoomTypesRow {
    pub r#id: String,
    pub r#unknown8: bool,
    pub r#unknown9: bool,
    pub r#unknown10: i64,
    pub r#unknown26: i64,
    pub r#unknown42: bool,
    pub r#icon: String,
    pub r#unknown51: bool,
    pub r#description: String,
    pub r#unknown60: Vec<String>,
    pub r#rooms: Vec<SanctumRoomsRef>,
    pub r#unknown92: String,
    pub r#unknown100: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct SanctumRoomTypesRef(pub usize);

impl Deref for SanctumRoomTypesRef {
    type Target = SanctumRoomTypesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_SanctumRoomTypes[self.0]
    }
}

impl SanctumRoomTypesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static SanctumRoomTypesRow {
        &TABLE_SanctumRoomTypes[self.0]
    }
    pub fn get(&self) -> &'static SanctumRoomTypesRow {
        &TABLE_SanctumRoomTypes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_SanctumRoomTypes.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static SanctumRoomTypesRow)> {
        TABLE_SanctumRoomTypes.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_SanctumRoomTypes.iter() {
            black_box(row);
        }
    }
}
