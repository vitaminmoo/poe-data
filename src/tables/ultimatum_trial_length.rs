#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_UltimatumTrialLength: LazyLock<Vec<UltimatumTrialLengthRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/ultimatumtriallength.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| UltimatumTrialLengthRow {
            r#min_area_level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#length: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(4..4 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct UltimatumTrialLengthRow {
    pub r#min_area_level: i32,
    pub r#length: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct UltimatumTrialLengthRef(pub usize);

impl Deref for UltimatumTrialLengthRef {
    type Target = UltimatumTrialLengthRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_UltimatumTrialLength[self.0]
    }
}

impl UltimatumTrialLengthRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static UltimatumTrialLengthRow {
        &TABLE_UltimatumTrialLength[self.0]
    }
    pub fn get(&self) -> &'static UltimatumTrialLengthRow {
        &TABLE_UltimatumTrialLength[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_UltimatumTrialLength.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static UltimatumTrialLengthRow)> {
        TABLE_UltimatumTrialLength.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_UltimatumTrialLength.iter() {
            black_box(row);
        }
    }
}
