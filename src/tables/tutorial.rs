#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_Tutorial: LazyLock<Vec<TutorialRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/tutorial.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| TutorialRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#ui_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#client_string: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ClientStringsRef::new(value as usize)
            },
            r#is_enabled: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(32).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown33: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(33..33 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown37: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(37..37 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 4)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i32_le())
                    .collect::<Vec<i32>>();
                values
            },
            r#unknown53: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(53..53 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown69: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(69..69 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown73: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(73..73 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 4)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i32_le())
                    .collect::<Vec<i32>>();
                values
            },
            r#unknown89: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(89).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown90: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(90).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown91: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(91..91 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct TutorialRow {
    pub r#id: String,
    pub r#ui_file: String,
    pub r#client_string: ClientStringsRef,
    pub r#is_enabled: bool,
    pub r#unknown33: i32,
    pub r#unknown37: Vec<i32>,
    pub r#unknown53: i64,
    pub r#unknown69: i32,
    pub r#unknown73: Vec<i32>,
    pub r#unknown89: bool,
    pub r#unknown90: bool,
    pub r#unknown91: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct TutorialRef(pub usize);

impl Deref for TutorialRef {
    type Target = TutorialRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_Tutorial[self.0]
    }
}

impl TutorialRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static TutorialRow {
        &TABLE_Tutorial[self.0]
    }
    pub fn get(&self) -> &'static TutorialRow {
        &TABLE_Tutorial[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Tutorial.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static TutorialRow)> {
        TABLE_Tutorial.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_Tutorial.iter() {
            black_box(row);
        }
    }
}
