#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_BlightStashTabLayout: LazyLock<Vec<BlightStashTabLayoutRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/blightstashtablayout.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| BlightStashTabLayoutRow {
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
            r#x_offset: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#y_offset: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(28..28 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#first_slot_index: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#width: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(36..36 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#height: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#slot_size: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(44..44 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown48: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(48).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown49: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(49).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown50: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(50).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown51: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(51).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct BlightStashTabLayoutRow {
    pub r#id: String,
    pub r#stored_item: BaseItemTypesRef,
    pub r#x_offset: i32,
    pub r#y_offset: i32,
    pub r#first_slot_index: i32,
    pub r#width: i32,
    pub r#height: i32,
    pub r#slot_size: i32,
    pub r#unknown48: bool,
    pub r#unknown49: bool,
    pub r#unknown50: bool,
    pub r#unknown51: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BlightStashTabLayoutRef(pub usize);

impl Deref for BlightStashTabLayoutRef {
    type Target = BlightStashTabLayoutRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_BlightStashTabLayout[self.0]
    }
}

impl BlightStashTabLayoutRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BlightStashTabLayoutRow {
        &TABLE_BlightStashTabLayout[self.0]
    }
    pub fn get(&self) -> &'static BlightStashTabLayoutRow {
        &TABLE_BlightStashTabLayout[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BlightStashTabLayout.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BlightStashTabLayoutRow)> {
        TABLE_BlightStashTabLayout.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_BlightStashTabLayout.iter() {
            black_box(row);
        }
    }
}
