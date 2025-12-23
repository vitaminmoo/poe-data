#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MapTiers: LazyLock<Vec<MapTiersRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/maptiers.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| MapTiersRow {
            r#tier: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(4..4 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#level2: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown12: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(12..12 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MapTiersRow {
    pub r#tier: i32,
    pub r#level: i32,
    pub r#level2: i32,
    pub r#unknown12: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MapTiersRef(pub usize);

impl Deref for MapTiersRef {
    type Target = MapTiersRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MapTiers[self.0]
    }
}

impl MapTiersRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MapTiersRow {
        &TABLE_MapTiers[self.0]
    }
    pub fn get(&self) -> &'static MapTiersRow {
        &TABLE_MapTiers[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MapTiers.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MapTiersRow)> {
        TABLE_MapTiers.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_MapTiers.iter() {
            black_box(row);
        }
    }
}
