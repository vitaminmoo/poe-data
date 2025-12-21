#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_FragmentStashSubPages: LazyLock<Vec<FragmentStashSubPagesRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/fragmentstashsubpages.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| FragmentStashSubPagesRow {
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
                r#icon: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(16..16 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#main_page: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(24..24 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    FragmentStashPagesRef::new(value as usize)
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct FragmentStashSubPagesRow {
    pub r#id: String,
    pub r#text: String,
    pub r#icon: String,
    pub r#main_page: FragmentStashPagesRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct FragmentStashSubPagesRef(pub usize);

impl Deref for FragmentStashSubPagesRef {
    type Target = FragmentStashSubPagesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_FragmentStashSubPages[self.0]
    }
}

impl FragmentStashSubPagesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static FragmentStashSubPagesRow {
        &TABLE_FragmentStashSubPages[self.0]
    }
    pub fn get(&self) -> &'static FragmentStashSubPagesRow {
        &TABLE_FragmentStashSubPages[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_FragmentStashSubPages
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static FragmentStashSubPagesRow)> {
        TABLE_FragmentStashSubPages
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
        for row in TABLE_FragmentStashSubPages.iter() {
            black_box(row);
        }
    }
}
