#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ComponentCharges: LazyLock<Vec<ComponentChargesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/componentcharges.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| ComponentChargesRow {
            r#base_item_types_key: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#max_charges: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#per_charge: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(12..12 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#max_charges2: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#per_charge2: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ComponentChargesRow {
    pub r#base_item_types_key: String,
    pub r#max_charges: i32,
    pub r#per_charge: i32,
    pub r#max_charges2: i32,
    pub r#per_charge2: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ComponentChargesRef(pub usize);

impl Deref for ComponentChargesRef {
    type Target = ComponentChargesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ComponentCharges[self.0]
    }
}

impl ComponentChargesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ComponentChargesRow {
        &TABLE_ComponentCharges[self.0]
    }
    pub fn get(&self) -> &'static ComponentChargesRow {
        &TABLE_ComponentCharges[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ComponentCharges.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ComponentChargesRow)> {
        TABLE_ComponentCharges.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_ComponentCharges.iter() {
            black_box(row);
        }
    }
}
