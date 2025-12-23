#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_KiracLevels: LazyLock<Vec<KiracLevelsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/kiraclevels.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| KiracLevelsRow {
            r#area_level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown4: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(4..4 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct KiracLevelsRow {
    pub r#area_level: i32,
    pub r#unknown4: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct KiracLevelsRef(pub usize);

impl Deref for KiracLevelsRef {
    type Target = KiracLevelsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_KiracLevels[self.0]
    }
}

impl KiracLevelsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static KiracLevelsRow {
        &TABLE_KiracLevels[self.0]
    }
    pub fn get(&self) -> &'static KiracLevelsRow {
        &TABLE_KiracLevels[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_KiracLevels.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static KiracLevelsRow)> {
        TABLE_KiracLevels.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_KiracLevels.iter() {
            black_box(row);
        }
    }
}
