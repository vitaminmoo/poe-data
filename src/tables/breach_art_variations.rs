#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_BreachArtVariations: LazyLock<Vec<BreachArtVariationsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/breachartvariations.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| BreachArtVariationsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown8: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
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
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(72..72 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown88: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(88..88 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown104: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(104..104 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown120: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(120..120 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown136: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(136..136 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct BreachArtVariationsRow {
    pub r#id: String,
    pub r#unknown8: i64,
    pub r#unknown24: i64,
    pub r#unknown40: i64,
    pub r#unknown56: i64,
    pub r#unknown72: i64,
    pub r#unknown88: i64,
    pub r#unknown104: i64,
    pub r#unknown120: (usize, usize),
    pub r#unknown136: i64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BreachArtVariationsRef(pub usize);

impl Deref for BreachArtVariationsRef {
    type Target = BreachArtVariationsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_BreachArtVariations[self.0]
    }
}

impl BreachArtVariationsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BreachArtVariationsRow {
        &TABLE_BreachArtVariations[self.0]
    }
    pub fn get(&self) -> &'static BreachArtVariationsRow {
        &TABLE_BreachArtVariations[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BreachArtVariations
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BreachArtVariationsRow)> {
        TABLE_BreachArtVariations
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
        for row in TABLE_BreachArtVariations.iter() {
            black_box(row);
        }
    }
}
