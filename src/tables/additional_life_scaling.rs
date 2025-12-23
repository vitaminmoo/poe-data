#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_AdditionalLifeScaling: LazyLock<Vec<AdditionalLifeScalingRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/additionallifescaling.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| AdditionalLifeScalingRow {
            r#int_id: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(4..4 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#dat_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(12..12 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct AdditionalLifeScalingRow {
    pub r#int_id: i32,
    pub r#id: String,
    pub r#dat_file: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct AdditionalLifeScalingRef(pub usize);

impl Deref for AdditionalLifeScalingRef {
    type Target = AdditionalLifeScalingRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_AdditionalLifeScaling[self.0]
    }
}

impl AdditionalLifeScalingRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static AdditionalLifeScalingRow {
        &TABLE_AdditionalLifeScaling[self.0]
    }
    pub fn get(&self) -> &'static AdditionalLifeScalingRow {
        &TABLE_AdditionalLifeScaling[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_AdditionalLifeScaling.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static AdditionalLifeScalingRow)> {
        TABLE_AdditionalLifeScaling.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_AdditionalLifeScaling.iter() {
            black_box(row);
        }
    }
}
