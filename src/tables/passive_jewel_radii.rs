#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_PassiveJewelRadii: LazyLock<Vec<PassiveJewelRadiiRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/passivejewelradii.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| PassiveJewelRadiiRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#ring_outer_radius: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#ring_inner_radius: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(12..12 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#radius: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown20: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct PassiveJewelRadiiRow {
    pub r#id: String,
    pub r#ring_outer_radius: i32,
    pub r#ring_inner_radius: i32,
    pub r#radius: i32,
    pub r#unknown20: i64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct PassiveJewelRadiiRef(pub usize);

impl Deref for PassiveJewelRadiiRef {
    type Target = PassiveJewelRadiiRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_PassiveJewelRadii[self.0]
    }
}

impl PassiveJewelRadiiRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static PassiveJewelRadiiRow {
        &TABLE_PassiveJewelRadii[self.0]
    }
    pub fn get(&self) -> &'static PassiveJewelRadiiRow {
        &TABLE_PassiveJewelRadii[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_PassiveJewelRadii.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static PassiveJewelRadiiRow)> {
        TABLE_PassiveJewelRadii.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_PassiveJewelRadii.iter() {
            black_box(row);
        }
    }
}
