#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ExtraTerrainFeatures: LazyLock<Vec<ExtraTerrainFeaturesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/extraterrainfeatures.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ExtraTerrainFeaturesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#arm_files: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#tdt_files: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown40: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(40..40 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown56: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(56..56 + 16).unwrap();
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
            r#unknown72: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(72..72 + 8).unwrap();
                let value = cell_bytes.get_i64_le();
                ExtraTerrainFeaturesRef::new(value as usize)
            },
            r#unknown80: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(80..80 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown96: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(96).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown97: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(97..97 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ExtraTerrainFeaturesRow {
    pub r#id: String,
    pub r#arm_files: Vec<String>,
    pub r#tdt_files: Vec<String>,
    pub r#unknown40: Vec<String>,
    pub r#unknown56: Vec<i32>,
    pub r#unknown72: ExtraTerrainFeaturesRef,
    pub r#unknown80: i64,
    pub r#unknown96: bool,
    pub r#unknown97: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ExtraTerrainFeaturesRef(pub usize);

impl Deref for ExtraTerrainFeaturesRef {
    type Target = ExtraTerrainFeaturesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ExtraTerrainFeatures[self.0]
    }
}

impl ExtraTerrainFeaturesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ExtraTerrainFeaturesRow {
        &TABLE_ExtraTerrainFeatures[self.0]
    }
    pub fn get(&self) -> &'static ExtraTerrainFeaturesRow {
        &TABLE_ExtraTerrainFeatures[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ExtraTerrainFeatures.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ExtraTerrainFeaturesRow)> {
        TABLE_ExtraTerrainFeatures.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_ExtraTerrainFeatures.iter() {
            black_box(row);
        }
    }
}
