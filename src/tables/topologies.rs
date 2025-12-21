#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_Topologies: LazyLock<Vec<TopologiesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/topologies.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| TopologiesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#dgr_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown20: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown24: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown28: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(28..28 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown44: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(44..44 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct TopologiesRow {
    pub r#id: String,
    pub r#dgr_file: String,
    pub r#unknown16: i32,
    pub r#unknown20: i32,
    pub r#unknown24: i32,
    pub r#unknown28: (usize, usize),
    pub r#unknown44: (usize, usize),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct TopologiesRef(pub usize);

impl Deref for TopologiesRef {
    type Target = TopologiesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_Topologies[self.0]
    }
}

impl TopologiesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static TopologiesRow {
        &TABLE_Topologies[self.0]
    }
    pub fn get(&self) -> &'static TopologiesRow {
        &TABLE_Topologies[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Topologies.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static TopologiesRow)> {
        TABLE_Topologies
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
        for row in TABLE_Topologies.iter() {
            black_box(row);
        }
    }
}
