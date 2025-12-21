#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ExpeditionRelics: LazyLock<Vec<ExpeditionRelicsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/expeditionrelics.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ExpeditionRelicsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#item_tag: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                TagsRef::new(value as usize)
            },
            r#ao_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(32..32 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#min_level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#max_level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(44..44 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ExpeditionRelicsRow {
    pub r#id: String,
    pub r#name: String,
    pub r#item_tag: TagsRef,
    pub r#ao_file: String,
    pub r#min_level: i32,
    pub r#max_level: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ExpeditionRelicsRef(pub usize);

impl Deref for ExpeditionRelicsRef {
    type Target = ExpeditionRelicsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ExpeditionRelics[self.0]
    }
}

impl ExpeditionRelicsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ExpeditionRelicsRow {
        &TABLE_ExpeditionRelics[self.0]
    }
    pub fn get(&self) -> &'static ExpeditionRelicsRow {
        &TABLE_ExpeditionRelics[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ExpeditionRelics
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ExpeditionRelicsRow)> {
        TABLE_ExpeditionRelics
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
        for row in TABLE_ExpeditionRelics.iter() {
            black_box(row);
        }
    }
}
