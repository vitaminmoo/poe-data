#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_EndgameSplinterInfoDisplay: LazyLock<Vec<EndgameSplinterInfoDisplayRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/endgamesplinterinfodisplay.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| EndgameSplinterInfoDisplayRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#text: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown20: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown24: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct EndgameSplinterInfoDisplayRow {
    pub r#id: String,
    pub r#text: String,
    pub r#unknown16: i32,
    pub r#unknown20: i32,
    pub r#unknown24: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct EndgameSplinterInfoDisplayRef(pub usize);

impl Deref for EndgameSplinterInfoDisplayRef {
    type Target = EndgameSplinterInfoDisplayRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_EndgameSplinterInfoDisplay[self.0]
    }
}

impl EndgameSplinterInfoDisplayRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static EndgameSplinterInfoDisplayRow {
        &TABLE_EndgameSplinterInfoDisplay[self.0]
    }
    pub fn get(&self) -> &'static EndgameSplinterInfoDisplayRow {
        &TABLE_EndgameSplinterInfoDisplay[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_EndgameSplinterInfoDisplay.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static EndgameSplinterInfoDisplayRow)> {
        TABLE_EndgameSplinterInfoDisplay.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_EndgameSplinterInfoDisplay.iter() {
            black_box(row);
        }
    }
}
