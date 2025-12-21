#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_Colours: LazyLock<Vec<ColoursRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/colours.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ColoursRow {
            r#item: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#red: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#green: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(12..12 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#blue: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#rgb_code: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(20..20 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ColoursRow {
    pub r#item: String,
    pub r#red: i32,
    pub r#green: i32,
    pub r#blue: i32,
    pub r#rgb_code: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ColoursRef(pub usize);

impl Deref for ColoursRef {
    type Target = ColoursRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_Colours[self.0]
    }
}

impl ColoursRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ColoursRow {
        &TABLE_Colours[self.0]
    }
    pub fn get(&self) -> &'static ColoursRow {
        &TABLE_Colours[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Colours.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ColoursRow)> {
        TABLE_Colours.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_Colours.iter() {
            black_box(row);
        }
    }
}
