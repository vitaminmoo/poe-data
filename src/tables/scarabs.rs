#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_Scarabs: LazyLock<Vec<ScarabsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/scarabs.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ScarabsRow {
            r#type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#tier: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(4..4 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#base_item_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ScarabsRow {
    pub r#type: i32,
    pub r#tier: i32,
    pub r#base_item_type: BaseItemTypesRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ScarabsRef(pub usize);

impl Deref for ScarabsRef {
    type Target = ScarabsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_Scarabs[self.0]
    }
}

impl ScarabsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ScarabsRow {
        &TABLE_Scarabs[self.0]
    }
    pub fn get(&self) -> &'static ScarabsRow {
        &TABLE_Scarabs[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Scarabs.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ScarabsRow)> {
        TABLE_Scarabs.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_Scarabs.iter() {
            black_box(row);
        }
    }
}
