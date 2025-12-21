#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_PreloadFromStats: LazyLock<Vec<PreloadFromStatsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/preloadfromstats.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| PreloadFromStatsRow {
            r#unknown0: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown16: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown32: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown48: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(48..48 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown64: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(64..64 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown80: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(80..80 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct PreloadFromStatsRow {
    pub r#unknown0: i64,
    pub r#unknown16: (usize, usize),
    pub r#unknown32: (usize, usize),
    pub r#unknown48: (usize, usize),
    pub r#unknown64: (usize, usize),
    pub r#unknown80: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct PreloadFromStatsRef(pub usize);

impl Deref for PreloadFromStatsRef {
    type Target = PreloadFromStatsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_PreloadFromStats[self.0]
    }
}

impl PreloadFromStatsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static PreloadFromStatsRow {
        &TABLE_PreloadFromStats[self.0]
    }
    pub fn get(&self) -> &'static PreloadFromStatsRow {
        &TABLE_PreloadFromStats[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_PreloadFromStats
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static PreloadFromStatsRow)> {
        TABLE_PreloadFromStats
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
        for row in TABLE_PreloadFromStats.iter() {
            black_box(row);
        }
    }
}
