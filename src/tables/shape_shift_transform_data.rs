#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ShapeShiftTransformData: LazyLock<Vec<ShapeShiftTransformDataRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/shapeshifttransformdata.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ShapeShiftTransformDataRow {
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
            r#unknown32: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(32..32 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown40: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown44: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(44..44 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown60: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(60).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown61: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(61).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown62: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(62).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ShapeShiftTransformDataRow {
    pub r#shape_shift_form: ShapeShiftFormsRef,
    pub r#unknown16: i64,
    pub r#unknown32: String,
    pub r#unknown40: i32,
    pub r#unknown44: i64,
    pub r#unknown60: bool,
    pub r#unknown61: bool,
    pub r#unknown62: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ShapeShiftTransformDataRef(pub usize);

impl Deref for ShapeShiftTransformDataRef {
    type Target = ShapeShiftTransformDataRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ShapeShiftTransformData[self.0]
    }
}

impl ShapeShiftTransformDataRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ShapeShiftTransformDataRow {
        &TABLE_ShapeShiftTransformData[self.0]
    }
    pub fn get(&self) -> &'static ShapeShiftTransformDataRow {
        &TABLE_ShapeShiftTransformData[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ShapeShiftTransformData.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ShapeShiftTransformDataRow)> {
        TABLE_ShapeShiftTransformData.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_ShapeShiftTransformData.iter() {
            black_box(row);
        }
    }
}
