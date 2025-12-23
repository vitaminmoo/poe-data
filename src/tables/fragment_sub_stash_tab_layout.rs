#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_FragmentSubStashTabLayout: LazyLock<Vec<FragmentSubStashTabLayoutRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/fragmentsubstashtablayout.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| FragmentSubStashTabLayoutRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#sub_stash: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                FragmentSubStashesRef::new(value as usize)
            },
            r#stored_item: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#unknown40: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown44: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(44..44 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown48: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(48..48 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown52: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(52..52 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown56: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(56..56 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown60: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(60..60 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown64: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(64..64 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown80: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(80).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown81: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(81..81 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown85: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(85..85 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown101: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(101..101 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown117: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(117..117 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct FragmentSubStashTabLayoutRow {
    pub r#id: String,
    pub r#sub_stash: FragmentSubStashesRef,
    pub r#stored_item: BaseItemTypesRef,
    pub r#unknown40: i32,
    pub r#unknown44: i32,
    pub r#unknown48: i32,
    pub r#unknown52: i32,
    pub r#unknown56: i32,
    pub r#unknown60: i32,
    pub r#unknown64: i64,
    pub r#unknown80: bool,
    pub r#unknown81: i32,
    pub r#unknown85: i64,
    pub r#unknown101: i64,
    pub r#unknown117: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct FragmentSubStashTabLayoutRef(pub usize);

impl Deref for FragmentSubStashTabLayoutRef {
    type Target = FragmentSubStashTabLayoutRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_FragmentSubStashTabLayout[self.0]
    }
}

impl FragmentSubStashTabLayoutRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static FragmentSubStashTabLayoutRow {
        &TABLE_FragmentSubStashTabLayout[self.0]
    }
    pub fn get(&self) -> &'static FragmentSubStashTabLayoutRow {
        &TABLE_FragmentSubStashTabLayout[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_FragmentSubStashTabLayout.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static FragmentSubStashTabLayoutRow)> {
        TABLE_FragmentSubStashTabLayout.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_FragmentSubStashTabLayout.iter() {
            black_box(row);
        }
    }
}
