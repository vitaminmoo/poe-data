#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ShapeShiftForms: LazyLock<Vec<ShapeShiftFormsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/shapeshiftforms.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| ShapeShiftFormsRow {
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
            r#stats_keys: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| StatsRef::new(value as usize)).collect()
            },
            r#stats_values: {
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
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(48..48 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown64: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(64..64 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown68: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(68..68 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown76: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(76..76 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown80: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(80..80 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown84: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(84..84 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown88: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(88..88 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#skill_stat: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(92..92 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
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
            r#status: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(113..113 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown121: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(121..121 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown125: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(125).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ShapeShiftFormsRow {
    pub r#id: String,
    pub r#ao_file: String,
    pub r#stats_keys: Vec<StatsRef>,
    pub r#stats_values: Vec<i32>,
    pub r#unknown48: i64,
    pub r#unknown64: i32,
    pub r#unknown68: String,
    pub r#unknown76: i32,
    pub r#unknown80: f32,
    pub r#unknown84: f32,
    pub r#unknown88: f32,
    pub r#skill_stat: StatsRef,
    pub r#unknown108: i32,
    pub r#unknown112: bool,
    pub r#status: String,
    pub r#unknown121: f32,
    pub r#unknown125: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ShapeShiftFormsRef(pub usize);

impl Deref for ShapeShiftFormsRef {
    type Target = ShapeShiftFormsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ShapeShiftForms[self.0]
    }
}

impl ShapeShiftFormsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ShapeShiftFormsRow {
        &TABLE_ShapeShiftForms[self.0]
    }
    pub fn get(&self) -> &'static ShapeShiftFormsRow {
        &TABLE_ShapeShiftForms[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ShapeShiftForms.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ShapeShiftFormsRow)> {
        TABLE_ShapeShiftForms.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_ShapeShiftForms.iter() {
            black_box(row);
        }
    }
}
