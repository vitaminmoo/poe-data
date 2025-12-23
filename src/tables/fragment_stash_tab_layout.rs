#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_FragmentStashTabLayout: LazyLock<Vec<FragmentStashTabLayoutRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/fragmentstashtablayout.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| FragmentStashTabLayoutRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#x_offset: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#y_offset: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(12..12 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#first_slot_index: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#width: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#height: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown28: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(28).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#tab: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(29..29 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#slot_size: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(33..33 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#hide_if_empty: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(37).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#subtab: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(38..38 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#stored_items: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(42..42 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| BaseItemTypesRef::new(value as usize)).collect()
            },
            r#unknown58: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(58).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown59: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(59..59 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown67: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(67..67 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct FragmentStashTabLayoutRow {
    pub r#id: String,
    pub r#x_offset: i32,
    pub r#y_offset: i32,
    pub r#first_slot_index: i32,
    pub r#width: i32,
    pub r#height: i32,
    pub r#unknown28: bool,
    pub r#tab: i32,
    pub r#slot_size: i32,
    pub r#hide_if_empty: bool,
    pub r#subtab: i32,
    pub r#stored_items: Vec<BaseItemTypesRef>,
    pub r#unknown58: bool,
    pub r#unknown59: String,
    pub r#unknown67: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct FragmentStashTabLayoutRef(pub usize);

impl Deref for FragmentStashTabLayoutRef {
    type Target = FragmentStashTabLayoutRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_FragmentStashTabLayout[self.0]
    }
}

impl FragmentStashTabLayoutRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static FragmentStashTabLayoutRow {
        &TABLE_FragmentStashTabLayout[self.0]
    }
    pub fn get(&self) -> &'static FragmentStashTabLayoutRow {
        &TABLE_FragmentStashTabLayout[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_FragmentStashTabLayout.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static FragmentStashTabLayoutRow)> {
        TABLE_FragmentStashTabLayout.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_FragmentStashTabLayout.iter() {
            black_box(row);
        }
    }
}
