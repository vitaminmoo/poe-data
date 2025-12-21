#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_RitualConstants: LazyLock<Vec<RitualConstantsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/ritualconstants.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| RitualConstantsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#value: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct RitualConstantsRow {
    pub r#id: String,
    pub r#value: f32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct RitualConstantsRef(pub usize);

impl Deref for RitualConstantsRef {
    type Target = RitualConstantsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_RitualConstants[self.0]
    }
}

impl RitualConstantsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static RitualConstantsRow {
        &TABLE_RitualConstants[self.0]
    }
    pub fn get(&self) -> &'static RitualConstantsRow {
        &TABLE_RitualConstants[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_RitualConstants
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static RitualConstantsRow)> {
        TABLE_RitualConstants
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_RitualConstants.iter() {
            black_box(row);
        }
    }
}
