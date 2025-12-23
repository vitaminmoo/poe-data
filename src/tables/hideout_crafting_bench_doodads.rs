#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_HideoutCraftingBenchDoodads: LazyLock<Vec<HideoutCraftingBenchDoodadsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/hideoutcraftingbenchdoodads.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| HideoutCraftingBenchDoodadsRow {
            r#unknown0: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct HideoutCraftingBenchDoodadsRow {
    pub r#unknown0: i64,
    pub r#unknown16: i64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct HideoutCraftingBenchDoodadsRef(pub usize);

impl Deref for HideoutCraftingBenchDoodadsRef {
    type Target = HideoutCraftingBenchDoodadsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_HideoutCraftingBenchDoodads[self.0]
    }
}

impl HideoutCraftingBenchDoodadsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static HideoutCraftingBenchDoodadsRow {
        &TABLE_HideoutCraftingBenchDoodads[self.0]
    }
    pub fn get(&self) -> &'static HideoutCraftingBenchDoodadsRow {
        &TABLE_HideoutCraftingBenchDoodads[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_HideoutCraftingBenchDoodads.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static HideoutCraftingBenchDoodadsRow)> {
        TABLE_HideoutCraftingBenchDoodads.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_HideoutCraftingBenchDoodads.iter() {
            black_box(row);
        }
    }
}
