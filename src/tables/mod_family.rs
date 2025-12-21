#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ModFamily: LazyLock<Vec<ModFamilyRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/modfamily.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ModFamilyRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ModFamilyRow {
    pub r#id: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ModFamilyRef(pub usize);

impl Deref for ModFamilyRef {
    type Target = ModFamilyRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ModFamily[self.0]
    }
}

impl ModFamilyRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ModFamilyRow {
        &TABLE_ModFamily[self.0]
    }
    pub fn get(&self) -> &'static ModFamilyRow {
        &TABLE_ModFamily[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ModFamily.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ModFamilyRow)> {
        TABLE_ModFamily
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn print_all_rows() {
        // Print all rows
        for row in TABLE_ModFamily.iter() {
            println!("{:?}", row);
        }
    }
}
