#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_HellscapeExperienceLevels: LazyLock<Vec<HellscapeExperienceLevelsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/hellscapeexperiencelevels.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| HellscapeExperienceLevelsRow {
            r#level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#experience: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(4..4 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct HellscapeExperienceLevelsRow {
    pub r#level: i32,
    pub r#experience: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct HellscapeExperienceLevelsRef(pub usize);

impl Deref for HellscapeExperienceLevelsRef {
    type Target = HellscapeExperienceLevelsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_HellscapeExperienceLevels[self.0]
    }
}

impl HellscapeExperienceLevelsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static HellscapeExperienceLevelsRow {
        &TABLE_HellscapeExperienceLevels[self.0]
    }
    pub fn get(&self) -> &'static HellscapeExperienceLevelsRow {
        &TABLE_HellscapeExperienceLevels[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_HellscapeExperienceLevels.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static HellscapeExperienceLevelsRow)> {
        TABLE_HellscapeExperienceLevels.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_HellscapeExperienceLevels.iter() {
            black_box(row);
        }
    }
}
