#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_CosmeticsEquipPanelMode: LazyLock<Vec<CosmeticsEquipPanelModeRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/cosmeticsequippanelmode.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| CosmeticsEquipPanelModeRow {
                r#id: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(0..0 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#unknown8: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(8..8 + 16).unwrap();
                    let count = cell_bytes.get_u64_le() as usize;
                    let offset = cell_bytes.get_u64_le() as usize;
                    // array_mutator column.array == true && column.type == 'array'
                    let values = (count, offset);
                    values
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct CosmeticsEquipPanelModeRow {
    pub r#id: String,
    pub r#unknown8: (usize, usize),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct CosmeticsEquipPanelModeRef(pub usize);

impl Deref for CosmeticsEquipPanelModeRef {
    type Target = CosmeticsEquipPanelModeRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_CosmeticsEquipPanelMode[self.0]
    }
}

impl CosmeticsEquipPanelModeRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static CosmeticsEquipPanelModeRow {
        &TABLE_CosmeticsEquipPanelMode[self.0]
    }
    pub fn get(&self) -> &'static CosmeticsEquipPanelModeRow {
        &TABLE_CosmeticsEquipPanelMode[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_CosmeticsEquipPanelMode
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static CosmeticsEquipPanelModeRow)> {
        TABLE_CosmeticsEquipPanelMode
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
        for row in TABLE_CosmeticsEquipPanelMode.iter() {
            black_box(row);
        }
    }
}
