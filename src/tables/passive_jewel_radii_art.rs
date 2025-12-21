#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_PassiveJewelRadiiArt: LazyLock<Vec<PassiveJewelRadiiArtRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/passivejewelradiiart.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| PassiveJewelRadiiArtRow {
                r#id: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(0..0 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#circle1: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(8..8 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#circle2: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(16..16 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#glow: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(24..24 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#unknown32: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(32..32 + 8).unwrap();
                    let value = cell_bytes.get_i64_le();
                    PassiveJewelRadiiArtRef::new(value as usize)
                },
                r#unknown40: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(40..40 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown44: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(44..44 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#inverse1: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(48..48 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#unknown56: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(56..56 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown60: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(60..60 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#inverse2: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(64..64 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct PassiveJewelRadiiArtRow {
    pub r#id: String,
    pub r#circle1: String,
    pub r#circle2: String,
    pub r#glow: String,
    pub r#unknown32: PassiveJewelRadiiArtRef,
    pub r#unknown40: i32,
    pub r#unknown44: i32,
    pub r#inverse1: String,
    pub r#unknown56: i32,
    pub r#unknown60: i32,
    pub r#inverse2: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct PassiveJewelRadiiArtRef(pub usize);

impl Deref for PassiveJewelRadiiArtRef {
    type Target = PassiveJewelRadiiArtRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_PassiveJewelRadiiArt[self.0]
    }
}

impl PassiveJewelRadiiArtRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static PassiveJewelRadiiArtRow {
        &TABLE_PassiveJewelRadiiArt[self.0]
    }
    pub fn get(&self) -> &'static PassiveJewelRadiiArtRow {
        &TABLE_PassiveJewelRadiiArt[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_PassiveJewelRadiiArt
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static PassiveJewelRadiiArtRow)> {
        TABLE_PassiveJewelRadiiArt
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
        for row in TABLE_PassiveJewelRadiiArt.iter() {
            black_box(row);
        }
    }
}
