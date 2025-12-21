#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_SummonedSpecificBarrels: LazyLock<Vec<SummonedSpecificBarrelsRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/summonedspecificbarrels.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| SummonedSpecificBarrelsRow {
                r#id: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(0..0 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#chest: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(8..8 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    ChestsRef::new(value as usize)
                },
                r#unknown24: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(24..24 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    value
                },
                r#unknown40: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(40..40 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    value
                },
                r#unknown56: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(56..56 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    value
                },
                r#unknown72: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(72..72 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct SummonedSpecificBarrelsRow {
    pub r#id: String,
    pub r#chest: ChestsRef,
    pub r#unknown24: i64,
    pub r#unknown40: i64,
    pub r#unknown56: i64,
    pub r#unknown72: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct SummonedSpecificBarrelsRef(pub usize);

impl Deref for SummonedSpecificBarrelsRef {
    type Target = SummonedSpecificBarrelsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_SummonedSpecificBarrels[self.0]
    }
}

impl SummonedSpecificBarrelsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static SummonedSpecificBarrelsRow {
        &TABLE_SummonedSpecificBarrels[self.0]
    }
    pub fn get(&self) -> &'static SummonedSpecificBarrelsRow {
        &TABLE_SummonedSpecificBarrels[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_SummonedSpecificBarrels
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static SummonedSpecificBarrelsRow)> {
        TABLE_SummonedSpecificBarrels
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
        for row in TABLE_SummonedSpecificBarrels.iter() {
            black_box(row);
        }
    }
}
