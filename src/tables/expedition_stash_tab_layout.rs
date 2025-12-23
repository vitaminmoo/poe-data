#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ExpeditionStashTabLayout: LazyLock<Vec<ExpeditionStashTabLayoutRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/expeditionstashtablayout.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ExpeditionStashTabLayoutRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#base_item_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ExpeditionStashTabLayoutRow {
    pub r#id: String,
    pub r#base_item_type: BaseItemTypesRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ExpeditionStashTabLayoutRef(pub usize);

impl Deref for ExpeditionStashTabLayoutRef {
    type Target = ExpeditionStashTabLayoutRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ExpeditionStashTabLayout[self.0]
    }
}

impl ExpeditionStashTabLayoutRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ExpeditionStashTabLayoutRow {
        &TABLE_ExpeditionStashTabLayout[self.0]
    }
    pub fn get(&self) -> &'static ExpeditionStashTabLayoutRow {
        &TABLE_ExpeditionStashTabLayout[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ExpeditionStashTabLayout.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ExpeditionStashTabLayoutRow)> {
        TABLE_ExpeditionStashTabLayout.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_ExpeditionStashTabLayout.iter() {
            black_box(row);
        }
    }
}
