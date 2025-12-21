#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_GrantedEffectLabels: LazyLock<Vec<GrantedEffectLabelsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/grantedeffectlabels.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| GrantedEffectLabelsRow {
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
            r#unknown16: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(16).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown17: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(17).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown18: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(18).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct GrantedEffectLabelsRow {
    pub r#id: String,
    pub r#text: String,
    pub r#unknown16: bool,
    pub r#unknown17: bool,
    pub r#unknown18: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct GrantedEffectLabelsRef(pub usize);

impl Deref for GrantedEffectLabelsRef {
    type Target = GrantedEffectLabelsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_GrantedEffectLabels[self.0]
    }
}

impl GrantedEffectLabelsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static GrantedEffectLabelsRow {
        &TABLE_GrantedEffectLabels[self.0]
    }
    pub fn get(&self) -> &'static GrantedEffectLabelsRow {
        &TABLE_GrantedEffectLabels[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_GrantedEffectLabels
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static GrantedEffectLabelsRow)> {
        TABLE_GrantedEffectLabels
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
        for row in TABLE_GrantedEffectLabels.iter() {
            println!("{:?}", row);
        }
    }
}
