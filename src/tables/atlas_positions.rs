#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_AtlasPositions: LazyLock<Vec<AtlasPositionsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/atlaspositions.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| AtlasPositionsRow {
            r#unknown0: {
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
            r#x: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#y: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(12..12 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct AtlasPositionsRow {
    pub r#unknown0: i32,
    pub r#unknown4: i32,
    pub r#x: f32,
    pub r#y: f32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct AtlasPositionsRef(pub usize);

impl Deref for AtlasPositionsRef {
    type Target = AtlasPositionsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_AtlasPositions[self.0]
    }
}

impl AtlasPositionsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static AtlasPositionsRow {
        &TABLE_AtlasPositions[self.0]
    }
    pub fn get(&self) -> &'static AtlasPositionsRow {
        &TABLE_AtlasPositions[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_AtlasPositions.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static AtlasPositionsRow)> {
        TABLE_AtlasPositions.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_AtlasPositions.iter() {
            black_box(row);
        }
    }
}
