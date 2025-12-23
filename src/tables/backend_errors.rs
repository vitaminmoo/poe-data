#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_BackendErrors: LazyLock<Vec<BackendErrorsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/backenderrors.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| BackendErrorsRow {
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
        })
        .collect()
});

#[derive(Debug)]
pub struct BackendErrorsRow {
    pub r#id: String,
    pub r#text: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BackendErrorsRef(pub usize);

impl Deref for BackendErrorsRef {
    type Target = BackendErrorsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_BackendErrors[self.0]
    }
}

impl BackendErrorsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BackendErrorsRow {
        &TABLE_BackendErrors[self.0]
    }
    pub fn get(&self) -> &'static BackendErrorsRow {
        &TABLE_BackendErrors[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BackendErrors.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BackendErrorsRow)> {
        TABLE_BackendErrors.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_BackendErrors.iter() {
            black_box(row);
        }
    }
}
