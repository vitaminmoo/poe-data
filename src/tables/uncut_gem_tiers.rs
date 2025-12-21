#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_UncutGemTiers: LazyLock<Vec<UncutGemTiersRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/uncutgemtiers.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| UncutGemTiersRow {
            r#base_item_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#tier: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#new_base_item_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct UncutGemTiersRow {
    pub r#base_item_type: BaseItemTypesRef,
    pub r#tier: i32,
    pub r#new_base_item_type: BaseItemTypesRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct UncutGemTiersRef(pub usize);

impl Deref for UncutGemTiersRef {
    type Target = UncutGemTiersRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_UncutGemTiers[self.0]
    }
}

impl UncutGemTiersRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static UncutGemTiersRow {
        &TABLE_UncutGemTiers[self.0]
    }
    pub fn get(&self) -> &'static UncutGemTiersRow {
        &TABLE_UncutGemTiers[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_UncutGemTiers.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static UncutGemTiersRow)> {
        TABLE_UncutGemTiers
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
        for row in TABLE_UncutGemTiers.iter() {
            black_box(row);
        }
    }
}
