#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_BestiaryNets: LazyLock<Vec<BestiaryNetsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/bestiarynets.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| BestiaryNetsRow {
            r#base_item_types_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#unknown16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#capture_min_level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#capture_max_level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#drop_min_level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(28..28 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#drop_max_level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown36: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(36..36 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#is_enabled: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(40).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct BestiaryNetsRow {
    pub r#base_item_types_key: BaseItemTypesRef,
    pub r#unknown16: i32,
    pub r#capture_min_level: i32,
    pub r#capture_max_level: i32,
    pub r#drop_min_level: i32,
    pub r#drop_max_level: i32,
    pub r#unknown36: i32,
    pub r#is_enabled: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BestiaryNetsRef(pub usize);

impl Deref for BestiaryNetsRef {
    type Target = BestiaryNetsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_BestiaryNets[self.0]
    }
}

impl BestiaryNetsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BestiaryNetsRow {
        &TABLE_BestiaryNets[self.0]
    }
    pub fn get(&self) -> &'static BestiaryNetsRow {
        &TABLE_BestiaryNets[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BestiaryNets.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BestiaryNetsRow)> {
        TABLE_BestiaryNets.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_BestiaryNets.iter() {
            black_box(row);
        }
    }
}
