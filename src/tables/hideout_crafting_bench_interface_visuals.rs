#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_HideoutCraftingBenchInterfaceVisuals: LazyLock<Vec<HideoutCraftingBenchInterfaceVisualsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/hideoutcraftingbenchinterfacevisuals.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| HideoutCraftingBenchInterfaceVisualsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown8: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown16: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown24: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(24..24 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown32: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(32..32 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown40: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(40..40 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct HideoutCraftingBenchInterfaceVisualsRow {
    pub r#id: String,
    pub r#unknown8: String,
    pub r#unknown16: String,
    pub r#unknown24: String,
    pub r#unknown32: String,
    pub r#unknown40: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct HideoutCraftingBenchInterfaceVisualsRef(pub usize);

impl Deref for HideoutCraftingBenchInterfaceVisualsRef {
    type Target = HideoutCraftingBenchInterfaceVisualsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_HideoutCraftingBenchInterfaceVisuals[self.0]
    }
}

impl HideoutCraftingBenchInterfaceVisualsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static HideoutCraftingBenchInterfaceVisualsRow {
        &TABLE_HideoutCraftingBenchInterfaceVisuals[self.0]
    }
    pub fn get(&self) -> &'static HideoutCraftingBenchInterfaceVisualsRow {
        &TABLE_HideoutCraftingBenchInterfaceVisuals[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_HideoutCraftingBenchInterfaceVisuals.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static HideoutCraftingBenchInterfaceVisualsRow)> {
        TABLE_HideoutCraftingBenchInterfaceVisuals.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_HideoutCraftingBenchInterfaceVisuals.iter() {
            black_box(row);
        }
    }
}
