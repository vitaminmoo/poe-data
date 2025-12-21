#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_GiftWrapArtVariations: LazyLock<Vec<GiftWrapArtVariationsRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/giftwrapartvariations.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| GiftWrapArtVariationsRow {
                r#width: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(0..0 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#height: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(4..4 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown8: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(8..8 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#item: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(12..12 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    BaseItemTypesRef::new(value as usize)
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct GiftWrapArtVariationsRow {
    pub r#width: i32,
    pub r#height: i32,
    pub r#unknown8: i32,
    pub r#item: BaseItemTypesRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct GiftWrapArtVariationsRef(pub usize);

impl Deref for GiftWrapArtVariationsRef {
    type Target = GiftWrapArtVariationsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_GiftWrapArtVariations[self.0]
    }
}

impl GiftWrapArtVariationsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static GiftWrapArtVariationsRow {
        &TABLE_GiftWrapArtVariations[self.0]
    }
    pub fn get(&self) -> &'static GiftWrapArtVariationsRow {
        &TABLE_GiftWrapArtVariations[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_GiftWrapArtVariations
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static GiftWrapArtVariationsRow)> {
        TABLE_GiftWrapArtVariations
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
        for row in TABLE_GiftWrapArtVariations.iter() {
            black_box(row);
        }
    }
}
