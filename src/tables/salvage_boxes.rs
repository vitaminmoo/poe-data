#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_SalvageBoxes: LazyLock<Vec<SalvageBoxesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/salvageboxes.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| SalvageBoxesRow {
            r#base_item_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown24: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(24..24 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct SalvageBoxesRow {
    pub r#base_item_type: BaseItemTypesRef,
    pub r#id: String,
    pub r#unknown24: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct SalvageBoxesRef(pub usize);

impl Deref for SalvageBoxesRef {
    type Target = SalvageBoxesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_SalvageBoxes[self.0]
    }
}

impl SalvageBoxesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static SalvageBoxesRow {
        &TABLE_SalvageBoxes[self.0]
    }
    pub fn get(&self) -> &'static SalvageBoxesRow {
        &TABLE_SalvageBoxes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_SalvageBoxes.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static SalvageBoxesRow)> {
        TABLE_SalvageBoxes.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_SalvageBoxes.iter() {
            black_box(row);
        }
    }
}
