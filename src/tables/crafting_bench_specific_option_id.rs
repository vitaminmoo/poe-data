#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_CraftingBenchSpecificOptionId: LazyLock<Vec<CraftingBenchSpecificOptionIdRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/craftingbenchspecificoptionid.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| CraftingBenchSpecificOptionIdRow {
                r#unknown0: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(0..0 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#unknown8: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(8..8 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct CraftingBenchSpecificOptionIdRow {
    pub r#unknown0: String,
    pub r#unknown8: i64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct CraftingBenchSpecificOptionIdRef(pub usize);

impl Deref for CraftingBenchSpecificOptionIdRef {
    type Target = CraftingBenchSpecificOptionIdRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_CraftingBenchSpecificOptionId[self.0]
    }
}

impl CraftingBenchSpecificOptionIdRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static CraftingBenchSpecificOptionIdRow {
        &TABLE_CraftingBenchSpecificOptionId[self.0]
    }
    pub fn get(&self) -> &'static CraftingBenchSpecificOptionIdRow {
        &TABLE_CraftingBenchSpecificOptionId[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_CraftingBenchSpecificOptionId
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs(
    ) -> impl Iterator<Item = (Self, &'static CraftingBenchSpecificOptionIdRow)> {
        TABLE_CraftingBenchSpecificOptionId
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
        for row in TABLE_CraftingBenchSpecificOptionId.iter() {
            black_box(row);
        }
    }
}
