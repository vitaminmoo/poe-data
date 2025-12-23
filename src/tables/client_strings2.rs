#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ClientStrings2: LazyLock<Vec<ClientStrings2Row>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/clientstrings2.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| ClientStrings2Row {
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
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown24: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(24..24 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#hash32: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown36: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(36..36 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown44: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(44..44 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ClientStrings2Row {
    pub r#id: String,
    pub r#text: String,
    pub r#unknown16: String,
    pub r#unknown24: String,
    pub r#hash32: i32,
    pub r#unknown36: String,
    pub r#unknown44: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ClientStrings2Ref(pub usize);

impl Deref for ClientStrings2Ref {
    type Target = ClientStrings2Row;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ClientStrings2[self.0]
    }
}

impl ClientStrings2Ref {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ClientStrings2Row {
        &TABLE_ClientStrings2[self.0]
    }
    pub fn get(&self) -> &'static ClientStrings2Row {
        &TABLE_ClientStrings2[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ClientStrings2.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ClientStrings2Row)> {
        TABLE_ClientStrings2.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_ClientStrings2.iter() {
            black_box(row);
        }
    }
}
