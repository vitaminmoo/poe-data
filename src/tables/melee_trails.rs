#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MeleeTrails: LazyLock<Vec<MeleeTrailsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/meleetrails.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| MeleeTrailsRow {
            r#epk_file1: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown8: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#epk_file2: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(12..12 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown20: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown24: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown28: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(28..28 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown32: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(32).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#ao_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(33..33 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown41: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(41).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown42: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(42..42 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MeleeTrailsRow {
    pub r#epk_file1: String,
    pub r#unknown8: i32,
    pub r#epk_file2: String,
    pub r#unknown20: i32,
    pub r#unknown24: i32,
    pub r#unknown28: i32,
    pub r#unknown32: bool,
    pub r#ao_file: String,
    pub r#unknown41: bool,
    pub r#unknown42: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MeleeTrailsRef(pub usize);

impl Deref for MeleeTrailsRef {
    type Target = MeleeTrailsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MeleeTrails[self.0]
    }
}

impl MeleeTrailsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MeleeTrailsRow {
        &TABLE_MeleeTrails[self.0]
    }
    pub fn get(&self) -> &'static MeleeTrailsRow {
        &TABLE_MeleeTrails[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MeleeTrails.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MeleeTrailsRow)> {
        TABLE_MeleeTrails.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_MeleeTrails.iter() {
            black_box(row);
        }
    }
}
