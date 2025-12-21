#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_SocketNotches: LazyLock<Vec<SocketNotchesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/socketnotches.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| SocketNotchesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#description: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#red_socket_image: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#blue_socket_image: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(24..24 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#green_socket_image: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(32..32 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct SocketNotchesRow {
    pub r#id: String,
    pub r#description: String,
    pub r#red_socket_image: String,
    pub r#blue_socket_image: String,
    pub r#green_socket_image: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct SocketNotchesRef(pub usize);

impl Deref for SocketNotchesRef {
    type Target = SocketNotchesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_SocketNotches[self.0]
    }
}

impl SocketNotchesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static SocketNotchesRow {
        &TABLE_SocketNotches[self.0]
    }
    pub fn get(&self) -> &'static SocketNotchesRow {
        &TABLE_SocketNotches[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_SocketNotches.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static SocketNotchesRow)> {
        TABLE_SocketNotches
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_SocketNotches.iter() {
            black_box(row);
        }
    }
}
