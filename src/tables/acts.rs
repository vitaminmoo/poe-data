#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_Acts: LazyLock<Vec<ActsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/acts.datc64")
        .unwrap()
        .clone();

    df.rows_iter()
        .map(|row| ActsRow {
            r#id: df
                .string_from_offset(row.get(0..8).unwrap().get_i32_le() as usize)
                .unwrap(),
            r#ui_title: row.get(8..16).unwrap().get_i32_le(),
            r#act_number: row.get(16..20).unwrap().get_i32_le(),
            r#is_end_game: row.get(40).unwrap().to_le() != 0,
            r#unknown_int: row.get(41..43).unwrap().get_i16_le(),

            r#unknown_foreign_array: df
                .array_from_offset(
                    row.get(53..59).unwrap().get_i32_le() as usize,
                    row.get(45..51).unwrap().get_i32_le() as usize,
                    16,
                )
                .unwrap()
                .iter()
                .map(|x| x.clone().get_i32_le())
                .collect(),
            r#description: df
                .string_from_offset(row.get(125..131).unwrap().get_i32_le() as usize)
                .unwrap(),
        })
        .collect()
});

#[derive(Debug)]
pub struct ActsRow {
    pub r#id: String,
    pub r#ui_title: i32,
    pub r#act_number: i32,
    pub r#is_end_game: bool,
    pub r#unknown_int: i16,
    pub r#unknown_foreign_array: Vec<i32>,
    pub r#description: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ActsRef(pub usize);

impl Deref for ActsRef {
    type Target = ActsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_Acts[self.0]
    }
}

impl ActsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ActsRow {
        &TABLE_Acts[self.0]
    }
    pub fn get(&self) -> &'static ActsRow {
        &TABLE_Acts[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Acts.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ActsRow)> {
        TABLE_Acts.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn print_acts() {
        // Print all rows
        for act in TABLE_Acts.iter() {
            println!("{:?}", act);
        }
    }
}
