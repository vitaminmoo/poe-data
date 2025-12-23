#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_EssenceType: LazyLock<Vec<EssenceTypeRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/essencetype.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| EssenceTypeRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#essence_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#is_corrupted_essence: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(12).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#words_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(13..13 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                WordsRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct EssenceTypeRow {
    pub r#id: String,
    pub r#essence_type: i32,
    pub r#is_corrupted_essence: bool,
    pub r#words_key: WordsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct EssenceTypeRef(pub usize);

impl Deref for EssenceTypeRef {
    type Target = EssenceTypeRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_EssenceType[self.0]
    }
}

impl EssenceTypeRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static EssenceTypeRow {
        &TABLE_EssenceType[self.0]
    }
    pub fn get(&self) -> &'static EssenceTypeRow {
        &TABLE_EssenceType[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_EssenceType.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static EssenceTypeRow)> {
        TABLE_EssenceType.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_EssenceType.iter() {
            black_box(row);
        }
    }
}
