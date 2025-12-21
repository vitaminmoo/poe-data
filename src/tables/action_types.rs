#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ActionTypes: LazyLock<Vec<ActionTypesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/actiontypes.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ActionTypesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown8: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 2).unwrap();
                let value = cell_bytes.get_u16_le();
                value
            },
            r#unknown10: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(10).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown11: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(11).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown12: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(12).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown13: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(13).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ActionTypesRow {
    pub r#id: String,
    pub r#unknown8: u16,
    pub r#unknown10: bool,
    pub r#unknown11: bool,
    pub r#unknown12: bool,
    pub r#unknown13: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ActionTypesRef(pub usize);

impl Deref for ActionTypesRef {
    type Target = ActionTypesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ActionTypes[self.0]
    }
}

impl ActionTypesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ActionTypesRow {
        &TABLE_ActionTypes[self.0]
    }
    pub fn get(&self) -> &'static ActionTypesRow {
        &TABLE_ActionTypes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ActionTypes.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ActionTypesRow)> {
        TABLE_ActionTypes
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
        for row in TABLE_ActionTypes.iter() {
            println!("{:?}", row);
        }
    }
}
