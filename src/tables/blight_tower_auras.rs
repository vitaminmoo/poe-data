#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_BlightTowerAuras: LazyLock<Vec<BlightTowerAurasRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/blighttowerauras.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| BlightTowerAurasRow {
            r#id: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#buff_definitions_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(4..4 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BuffDefinitionsRef::new(value as usize)
            },
            r#unknown20: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#misc_animated_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MiscAnimatedRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct BlightTowerAurasRow {
    pub r#id: i32,
    pub r#buff_definitions_key: BuffDefinitionsRef,
    pub r#unknown20: i32,
    pub r#misc_animated_key: MiscAnimatedRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BlightTowerAurasRef(pub usize);

impl Deref for BlightTowerAurasRef {
    type Target = BlightTowerAurasRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_BlightTowerAuras[self.0]
    }
}

impl BlightTowerAurasRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BlightTowerAurasRow {
        &TABLE_BlightTowerAuras[self.0]
    }
    pub fn get(&self) -> &'static BlightTowerAurasRow {
        &TABLE_BlightTowerAuras[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BlightTowerAuras
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BlightTowerAurasRow)> {
        TABLE_BlightTowerAuras
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
        for row in TABLE_BlightTowerAuras.iter() {
            black_box(row);
        }
    }
}
