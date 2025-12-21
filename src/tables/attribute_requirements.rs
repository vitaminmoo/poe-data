#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_AttributeRequirements: LazyLock<Vec<AttributeRequirementsRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/attributerequirements.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| AttributeRequirementsRow {
                r#base_item_type: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(0..0 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    BaseItemTypesRef::new(value as usize)
                },
                r#req_str: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(16..16 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#req_int: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(20..20 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#req_dex: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(24..24 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct AttributeRequirementsRow {
    pub r#base_item_type: BaseItemTypesRef,
    pub r#req_str: i32,
    pub r#req_int: i32,
    pub r#req_dex: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct AttributeRequirementsRef(pub usize);

impl Deref for AttributeRequirementsRef {
    type Target = AttributeRequirementsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_AttributeRequirements[self.0]
    }
}

impl AttributeRequirementsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static AttributeRequirementsRow {
        &TABLE_AttributeRequirements[self.0]
    }
    pub fn get(&self) -> &'static AttributeRequirementsRow {
        &TABLE_AttributeRequirements[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_AttributeRequirements
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static AttributeRequirementsRow)> {
        TABLE_AttributeRequirements
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
        for row in TABLE_AttributeRequirements.iter() {
            black_box(row);
        }
    }
}
