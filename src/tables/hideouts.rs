#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_Hideouts: LazyLock<Vec<HideoutsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/hideouts.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| HideoutsRow {
            r#hideout_area: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                WorldAreasRef::new(value as usize)
            },
            r#hash16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 2).unwrap();
                let value = cell_bytes.get_i16_le();
                value
            },
            r#hideout_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(18..18 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#claim_side_area: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(26..26 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                WorldAreasRef::new(value as usize)
            },
            r#hideout_image: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(42..42 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#is_enabled: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(50).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown51: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(51).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown52: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(52..52 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 4)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i32_le())
                    .collect::<Vec<i32>>();
                values
            },
            r#unknown68: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(68).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#mtx_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(69..69 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MtxTypesRef::new(value as usize)
            },
            r#error_message: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(85..85 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct HideoutsRow {
    pub r#hideout_area: WorldAreasRef,
    pub r#hash16: i16,
    pub r#hideout_file: String,
    pub r#claim_side_area: WorldAreasRef,
    pub r#hideout_image: String,
    pub r#is_enabled: bool,
    pub r#unknown51: bool,
    pub r#unknown52: Vec<i32>,
    pub r#unknown68: bool,
    pub r#mtx_type: MtxTypesRef,
    pub r#error_message: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct HideoutsRef(pub usize);

impl Deref for HideoutsRef {
    type Target = HideoutsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_Hideouts[self.0]
    }
}

impl HideoutsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static HideoutsRow {
        &TABLE_Hideouts[self.0]
    }
    pub fn get(&self) -> &'static HideoutsRow {
        &TABLE_Hideouts[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Hideouts.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static HideoutsRow)> {
        TABLE_Hideouts.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_Hideouts.iter() {
            black_box(row);
        }
    }
}
