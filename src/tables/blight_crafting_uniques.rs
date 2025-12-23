#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_BlightCraftingUniques: LazyLock<Vec<BlightCraftingUniquesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/blightcraftinguniques.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| BlightCraftingUniquesRow {
            r#words_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                WordsRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct BlightCraftingUniquesRow {
    pub r#words_key: WordsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BlightCraftingUniquesRef(pub usize);

impl Deref for BlightCraftingUniquesRef {
    type Target = BlightCraftingUniquesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_BlightCraftingUniques[self.0]
    }
}

impl BlightCraftingUniquesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BlightCraftingUniquesRow {
        &TABLE_BlightCraftingUniques[self.0]
    }
    pub fn get(&self) -> &'static BlightCraftingUniquesRow {
        &TABLE_BlightCraftingUniques[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BlightCraftingUniques.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BlightCraftingUniquesRow)> {
        TABLE_BlightCraftingUniques.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_BlightCraftingUniques.iter() {
            black_box(row);
        }
    }
}
