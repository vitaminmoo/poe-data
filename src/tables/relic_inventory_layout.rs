#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_RelicInventoryLayout: LazyLock<Vec<RelicInventoryLayoutRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/relicinventorylayout.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| RelicInventoryLayoutRow {
                r#unknown0: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(0..0 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown4: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(4..4 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown8: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(8..8 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#requirement: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(12..12 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct RelicInventoryLayoutRow {
    pub r#unknown0: i32,
    pub r#unknown4: i32,
    pub r#unknown8: i32,
    pub r#requirement: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct RelicInventoryLayoutRef(pub usize);

impl Deref for RelicInventoryLayoutRef {
    type Target = RelicInventoryLayoutRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_RelicInventoryLayout[self.0]
    }
}

impl RelicInventoryLayoutRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static RelicInventoryLayoutRow {
        &TABLE_RelicInventoryLayout[self.0]
    }
    pub fn get(&self) -> &'static RelicInventoryLayoutRow {
        &TABLE_RelicInventoryLayout[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_RelicInventoryLayout
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static RelicInventoryLayoutRow)> {
        TABLE_RelicInventoryLayout
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
        for row in TABLE_RelicInventoryLayout.iter() {
            black_box(row);
        }
    }
}
