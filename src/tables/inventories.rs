#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_Inventories: LazyLock<Vec<InventoriesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/inventories.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| InventoriesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#inventory_id_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#inventory_type_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(12..12 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                InventoryType::from_repr(value as usize)
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
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(18..18 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown22: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(22).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown23: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(23..23 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct InventoriesRow {
    pub r#id: String,
    pub r#inventory_id_key: i32,
    pub r#inventory_type_key: Option<InventoryType>,
    pub r#unknown16: bool,
    pub r#unknown17: bool,
    pub r#unknown18: i32,
    pub r#unknown22: bool,
    pub r#unknown23: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct InventoriesRef(pub usize);

impl Deref for InventoriesRef {
    type Target = InventoriesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_Inventories[self.0]
    }
}

impl InventoriesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static InventoriesRow {
        &TABLE_Inventories[self.0]
    }
    pub fn get(&self) -> &'static InventoriesRow {
        &TABLE_Inventories[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Inventories.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static InventoriesRow)> {
        TABLE_Inventories.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_Inventories.iter() {
            black_box(row);
        }
    }
}
