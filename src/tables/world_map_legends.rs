#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_WorldMapLegends: LazyLock<Vec<WorldMapLegendsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/worldmaplegends.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| WorldMapLegendsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#icon: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown24: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown28: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(28..28 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#acts: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values
                    .into_iter()
                    .map(|value| ActsRef::new(value as usize))
                    .collect()
            },
            r#unknown48: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(48).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown49: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(49..49 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct WorldMapLegendsRow {
    pub r#id: String,
    pub r#icon: String,
    pub r#name: String,
    pub r#unknown24: i32,
    pub r#unknown28: i32,
    pub r#acts: Vec<ActsRef>,
    pub r#unknown48: bool,
    pub r#unknown49: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct WorldMapLegendsRef(pub usize);

impl Deref for WorldMapLegendsRef {
    type Target = WorldMapLegendsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_WorldMapLegends[self.0]
    }
}

impl WorldMapLegendsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static WorldMapLegendsRow {
        &TABLE_WorldMapLegends[self.0]
    }
    pub fn get(&self) -> &'static WorldMapLegendsRow {
        &TABLE_WorldMapLegends[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_WorldMapLegends
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static WorldMapLegendsRow)> {
        TABLE_WorldMapLegends
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
        for row in TABLE_WorldMapLegends.iter() {
            black_box(row);
        }
    }
}
