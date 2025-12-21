#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_HeistDoors: LazyLock<Vec<HeistDoorsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/heistdoors.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| HeistDoorsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown8: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#heist_jobs_key1: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                HeistJobsRef::new(value as usize)
            },
            r#heist_jobs_key2: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                HeistJobsRef::new(value as usize)
            },
            r#unknown48: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(48..48 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown56: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(56..56 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown72: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(72..72 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown88: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(88..88 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#heist_areas_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(92..92 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                HeistAreasRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct HeistDoorsRow {
    pub r#id: String,
    pub r#unknown8: String,
    pub r#heist_jobs_key1: HeistJobsRef,
    pub r#heist_jobs_key2: HeistJobsRef,
    pub r#unknown48: String,
    pub r#unknown56: Vec<String>,
    pub r#unknown72: Vec<String>,
    pub r#unknown88: i32,
    pub r#heist_areas_key: HeistAreasRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct HeistDoorsRef(pub usize);

impl Deref for HeistDoorsRef {
    type Target = HeistDoorsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_HeistDoors[self.0]
    }
}

impl HeistDoorsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static HeistDoorsRow {
        &TABLE_HeistDoors[self.0]
    }
    pub fn get(&self) -> &'static HeistDoorsRow {
        &TABLE_HeistDoors[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_HeistDoors.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static HeistDoorsRow)> {
        TABLE_HeistDoors
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
        for row in TABLE_HeistDoors.iter() {
            black_box(row);
        }
    }
}
