#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_HideoutDoodads: LazyLock<Vec<HideoutDoodadsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/hideoutdoodads.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| HideoutDoodadsRow {
            r#mtx_type_game_specific: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MtxTypeGameSpecificRef::new(value as usize)
            },
            r#variation_ao_files: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown32: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
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
            r#unknown48: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(48).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown49: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(49..49 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown57: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(57).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown58: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(58).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown59: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(59..59 + 16).unwrap();
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
            },
            r#unknown75: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(75).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown76: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(76..76 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown92: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(92..92 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown108: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(108..108 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown112: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(112).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown113: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(113..113 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown129: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(129).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown130: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(130..130 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown146: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(146..146 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown162: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(162..162 + 8).unwrap();
                let value = cell_bytes.get_i64_le();
                HideoutDoodadsRef::new(value as usize)
            },
            r#unknown170: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(170).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown171: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(171).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct HideoutDoodadsRow {
    pub r#mtx_type_game_specific: MtxTypeGameSpecificRef,
    pub r#variation_ao_files: Vec<String>,
    pub r#unknown32: Vec<i32>,
    pub r#unknown48: bool,
    pub r#unknown49: String,
    pub r#unknown57: bool,
    pub r#unknown58: bool,
    pub r#unknown59: Vec<i64>,
    pub r#unknown75: bool,
    pub r#unknown76: i64,
    pub r#unknown92: i64,
    pub r#unknown108: i32,
    pub r#unknown112: bool,
    pub r#unknown113: i64,
    pub r#unknown129: bool,
    pub r#unknown130: i64,
    pub r#unknown146: Vec<String>,
    pub r#unknown162: HideoutDoodadsRef,
    pub r#unknown170: bool,
    pub r#unknown171: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct HideoutDoodadsRef(pub usize);

impl Deref for HideoutDoodadsRef {
    type Target = HideoutDoodadsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_HideoutDoodads[self.0]
    }
}

impl HideoutDoodadsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static HideoutDoodadsRow {
        &TABLE_HideoutDoodads[self.0]
    }
    pub fn get(&self) -> &'static HideoutDoodadsRow {
        &TABLE_HideoutDoodads[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_HideoutDoodads.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static HideoutDoodadsRow)> {
        TABLE_HideoutDoodads.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_HideoutDoodads.iter() {
            black_box(row);
        }
    }
}
