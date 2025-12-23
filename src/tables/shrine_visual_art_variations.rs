#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ShrineVisualArtVariations: LazyLock<Vec<ShrineVisualArtVariationsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/shrinevisualartvariations.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ShrineVisualArtVariationsRow {
            r#unknown0: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown16: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(16).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown17: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(17..17 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown33: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(33).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown34: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(34).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown35: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(35).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ShrineVisualArtVariationsRow {
    pub r#unknown0: i64,
    pub r#unknown16: bool,
    pub r#unknown17: i64,
    pub r#unknown33: bool,
    pub r#unknown34: bool,
    pub r#unknown35: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ShrineVisualArtVariationsRef(pub usize);

impl Deref for ShrineVisualArtVariationsRef {
    type Target = ShrineVisualArtVariationsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ShrineVisualArtVariations[self.0]
    }
}

impl ShrineVisualArtVariationsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ShrineVisualArtVariationsRow {
        &TABLE_ShrineVisualArtVariations[self.0]
    }
    pub fn get(&self) -> &'static ShrineVisualArtVariationsRow {
        &TABLE_ShrineVisualArtVariations[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ShrineVisualArtVariations.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ShrineVisualArtVariationsRow)> {
        TABLE_ShrineVisualArtVariations.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_ShrineVisualArtVariations.iter() {
            black_box(row);
        }
    }
}
