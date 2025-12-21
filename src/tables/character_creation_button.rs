#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_CharacterCreationButton: LazyLock<Vec<CharacterCreationButtonRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/charactercreationbutton.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| CharacterCreationButtonRow {
                r#unknown0: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(0..0 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
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
                r#unknown32: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(32..32 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#unknown40: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(40..40 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct CharacterCreationButtonRow {
    pub r#unknown0: i64,
    pub r#unknown16: String,
    pub r#unknown24: String,
    pub r#unknown32: String,
    pub r#unknown40: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct CharacterCreationButtonRef(pub usize);

impl Deref for CharacterCreationButtonRef {
    type Target = CharacterCreationButtonRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_CharacterCreationButton[self.0]
    }
}

impl CharacterCreationButtonRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static CharacterCreationButtonRow {
        &TABLE_CharacterCreationButton[self.0]
    }
    pub fn get(&self) -> &'static CharacterCreationButtonRow {
        &TABLE_CharacterCreationButton[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_CharacterCreationButton
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static CharacterCreationButtonRow)> {
        TABLE_CharacterCreationButton
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
        for row in TABLE_CharacterCreationButton.iter() {
            black_box(row);
        }
    }
}
