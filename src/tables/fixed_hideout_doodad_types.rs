#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_FixedHideoutDoodadTypes: LazyLock<Vec<FixedHideoutDoodadTypesRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/fixedhideoutdoodadtypes.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| FixedHideoutDoodadTypesRow {
                r#id: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(0..0 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#hideout_doodads: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(8..8 + 16).unwrap();
                    let count = cell_bytes.get_u64_le() as usize;
                    let offset = cell_bytes.get_u64_le() as usize;
                    // array_mutator column.array == true && column.type else
                    let values = df
                        .array_from_offset(offset, count, 16)
                        .unwrap()
                        .iter()
                        .map(|x| x.clone().get_i64_le())
                        .collect::<Vec<i64>>();
                    values
                        .into_iter()
                        .map(|value| HideoutDoodadsRef::new(value as usize))
                        .collect()
                },
                r#base_type_hideout_doodads: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(24..24 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    HideoutDoodadsRef::new(value as usize)
                },
                r#unknown40: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(40..40 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown44: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(44..44 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown48: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(48).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#unknown49: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(49..49 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    value
                },
                r#unknown65: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(65..65 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct FixedHideoutDoodadTypesRow {
    pub r#id: String,
    pub r#hideout_doodads: Vec<HideoutDoodadsRef>,
    pub r#base_type_hideout_doodads: HideoutDoodadsRef,
    pub r#unknown40: i32,
    pub r#unknown44: i32,
    pub r#unknown48: bool,
    pub r#unknown49: i64,
    pub r#unknown65: i64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct FixedHideoutDoodadTypesRef(pub usize);

impl Deref for FixedHideoutDoodadTypesRef {
    type Target = FixedHideoutDoodadTypesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_FixedHideoutDoodadTypes[self.0]
    }
}

impl FixedHideoutDoodadTypesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static FixedHideoutDoodadTypesRow {
        &TABLE_FixedHideoutDoodadTypes[self.0]
    }
    pub fn get(&self) -> &'static FixedHideoutDoodadTypesRow {
        &TABLE_FixedHideoutDoodadTypes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_FixedHideoutDoodadTypes
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static FixedHideoutDoodadTypesRow)> {
        TABLE_FixedHideoutDoodadTypes
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
        for row in TABLE_FixedHideoutDoodadTypes.iter() {
            black_box(row);
        }
    }
}
