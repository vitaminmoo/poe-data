#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_EnvironmentTransitions: LazyLock<Vec<EnvironmentTransitionsRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/environmenttransitions.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| EnvironmentTransitionsRow {
                r#id: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(0..0 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#ot_files: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(8..8 + 16).unwrap();
                    let count = cell_bytes.get_u64_le() as usize;
                    let offset = cell_bytes.get_u64_le() as usize;
                    // array_mutator column.array == true && column.type == 'string'
                    let values = df.strings_from_offset(offset, count).unwrap();
                    values
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct EnvironmentTransitionsRow {
    pub r#id: String,
    pub r#ot_files: Vec<String>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct EnvironmentTransitionsRef(pub usize);

impl Deref for EnvironmentTransitionsRef {
    type Target = EnvironmentTransitionsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_EnvironmentTransitions[self.0]
    }
}

impl EnvironmentTransitionsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static EnvironmentTransitionsRow {
        &TABLE_EnvironmentTransitions[self.0]
    }
    pub fn get(&self) -> &'static EnvironmentTransitionsRow {
        &TABLE_EnvironmentTransitions[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_EnvironmentTransitions
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static EnvironmentTransitionsRow)> {
        TABLE_EnvironmentTransitions
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
        for row in TABLE_EnvironmentTransitions.iter() {
            black_box(row);
        }
    }
}
