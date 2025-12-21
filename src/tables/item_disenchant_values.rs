#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ItemDisenchantValues: LazyLock<Vec<ItemDisenchantValuesRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/itemdisenchantvalues.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| ItemDisenchantValuesRow {
                r#rarity: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(0..0 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    RarityRef::new(value as usize)
                },
                r#base_item_type: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(16..16 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    BaseItemTypesRef::new(value as usize)
                },
                r#base_value: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(32..32 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct ItemDisenchantValuesRow {
    pub r#rarity: RarityRef,
    pub r#base_item_type: BaseItemTypesRef,
    pub r#base_value: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ItemDisenchantValuesRef(pub usize);

impl Deref for ItemDisenchantValuesRef {
    type Target = ItemDisenchantValuesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ItemDisenchantValues[self.0]
    }
}

impl ItemDisenchantValuesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ItemDisenchantValuesRow {
        &TABLE_ItemDisenchantValues[self.0]
    }
    pub fn get(&self) -> &'static ItemDisenchantValuesRow {
        &TABLE_ItemDisenchantValues[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ItemDisenchantValues
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ItemDisenchantValuesRow)> {
        TABLE_ItemDisenchantValues
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
        for row in TABLE_ItemDisenchantValues.iter() {
            black_box(row);
        }
    }
}
