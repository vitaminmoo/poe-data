#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ShapeShiftVisualIdentity: LazyLock<Vec<ShapeShiftVisualIdentityRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/shapeshiftvisualidentity.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ShapeShiftVisualIdentityRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#ao_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ShapeShiftVisualIdentityRow {
    pub r#id: String,
    pub r#ao_file: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ShapeShiftVisualIdentityRef(pub usize);

impl Deref for ShapeShiftVisualIdentityRef {
    type Target = ShapeShiftVisualIdentityRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ShapeShiftVisualIdentity[self.0]
    }
}

impl ShapeShiftVisualIdentityRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ShapeShiftVisualIdentityRow {
        &TABLE_ShapeShiftVisualIdentity[self.0]
    }
    pub fn get(&self) -> &'static ShapeShiftVisualIdentityRow {
        &TABLE_ShapeShiftVisualIdentity[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ShapeShiftVisualIdentity.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ShapeShiftVisualIdentityRow)> {
        TABLE_ShapeShiftVisualIdentity.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_ShapeShiftVisualIdentity.iter() {
            black_box(row);
        }
    }
}
