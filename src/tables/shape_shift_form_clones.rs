#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ShapeShiftFormClones: LazyLock<Vec<ShapeShiftFormClonesRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/shapeshiftformclones.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| ShapeShiftFormClonesRow {
                r#shape_shift_form: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(0..0 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    ShapeShiftFormsRef::new(value as usize)
                },
                r#unknown16: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(16..16 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    value
                },
                r#metadata: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(32..32 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#metadata2: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(40..40 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct ShapeShiftFormClonesRow {
    pub r#shape_shift_form: ShapeShiftFormsRef,
    pub r#unknown16: i64,
    pub r#metadata: String,
    pub r#metadata2: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ShapeShiftFormClonesRef(pub usize);

impl Deref for ShapeShiftFormClonesRef {
    type Target = ShapeShiftFormClonesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ShapeShiftFormClones[self.0]
    }
}

impl ShapeShiftFormClonesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ShapeShiftFormClonesRow {
        &TABLE_ShapeShiftFormClones[self.0]
    }
    pub fn get(&self) -> &'static ShapeShiftFormClonesRow {
        &TABLE_ShapeShiftFormClones[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ShapeShiftFormClones
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ShapeShiftFormClonesRow)> {
        TABLE_ShapeShiftFormClones
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
        for row in TABLE_ShapeShiftFormClones.iter() {
            black_box(row);
        }
    }
}
