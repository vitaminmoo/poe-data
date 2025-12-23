#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_SentinelStorageLayout: LazyLock<Vec<SentinelStorageLayoutRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/sentinelstoragelayout.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| SentinelStorageLayoutRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#stored_item: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#drone_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                DroneTypesRef::new(value as usize)
            },
            r#unknown40: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(40).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#tab_icon: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(41..41 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#x_offset: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(49..49 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#y_offset: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(53..53 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown57: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(57..57 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown61: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(61..61 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#width: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(65..65 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#height: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(69..69 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#slot_size: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(73..73 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown77: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(77..77 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ItemClassesRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct SentinelStorageLayoutRow {
    pub r#id: String,
    pub r#stored_item: BaseItemTypesRef,
    pub r#drone_type: DroneTypesRef,
    pub r#unknown40: bool,
    pub r#tab_icon: String,
    pub r#x_offset: i32,
    pub r#y_offset: i32,
    pub r#unknown57: i32,
    pub r#unknown61: i32,
    pub r#width: i32,
    pub r#height: i32,
    pub r#slot_size: i32,
    pub r#unknown77: ItemClassesRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct SentinelStorageLayoutRef(pub usize);

impl Deref for SentinelStorageLayoutRef {
    type Target = SentinelStorageLayoutRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_SentinelStorageLayout[self.0]
    }
}

impl SentinelStorageLayoutRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static SentinelStorageLayoutRow {
        &TABLE_SentinelStorageLayout[self.0]
    }
    pub fn get(&self) -> &'static SentinelStorageLayoutRow {
        &TABLE_SentinelStorageLayout[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_SentinelStorageLayout.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static SentinelStorageLayoutRow)> {
        TABLE_SentinelStorageLayout.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_SentinelStorageLayout.iter() {
            black_box(row);
        }
    }
}
