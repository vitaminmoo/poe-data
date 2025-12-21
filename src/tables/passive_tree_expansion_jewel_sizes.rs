#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_PassiveTreeExpansionJewelSizes: LazyLock<Vec<PassiveTreeExpansionJewelSizesRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/passivetreeexpansionjewelsizes.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| PassiveTreeExpansionJewelSizesRow {
                r#name: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(0..0 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct PassiveTreeExpansionJewelSizesRow {
    pub r#name: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct PassiveTreeExpansionJewelSizesRef(pub usize);

impl Deref for PassiveTreeExpansionJewelSizesRef {
    type Target = PassiveTreeExpansionJewelSizesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_PassiveTreeExpansionJewelSizes[self.0]
    }
}

impl PassiveTreeExpansionJewelSizesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static PassiveTreeExpansionJewelSizesRow {
        &TABLE_PassiveTreeExpansionJewelSizes[self.0]
    }
    pub fn get(&self) -> &'static PassiveTreeExpansionJewelSizesRow {
        &TABLE_PassiveTreeExpansionJewelSizes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_PassiveTreeExpansionJewelSizes
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs(
    ) -> impl Iterator<Item = (Self, &'static PassiveTreeExpansionJewelSizesRow)> {
        TABLE_PassiveTreeExpansionJewelSizes
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
        for row in TABLE_PassiveTreeExpansionJewelSizes.iter() {
            black_box(row);
        }
    }
}
