#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_DelveResourcePerLevel: LazyLock<Vec<DelveResourcePerLevelRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/delveresourceperlevel.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| DelveResourcePerLevelRow {
            r#area_level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#sulphite: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(4..4 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct DelveResourcePerLevelRow {
    pub r#area_level: i32,
    pub r#sulphite: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct DelveResourcePerLevelRef(pub usize);

impl Deref for DelveResourcePerLevelRef {
    type Target = DelveResourcePerLevelRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_DelveResourcePerLevel[self.0]
    }
}

impl DelveResourcePerLevelRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static DelveResourcePerLevelRow {
        &TABLE_DelveResourcePerLevel[self.0]
    }
    pub fn get(&self) -> &'static DelveResourcePerLevelRow {
        &TABLE_DelveResourcePerLevel[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_DelveResourcePerLevel.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static DelveResourcePerLevelRow)> {
        TABLE_DelveResourcePerLevel.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_DelveResourcePerLevel.iter() {
            black_box(row);
        }
    }
}
