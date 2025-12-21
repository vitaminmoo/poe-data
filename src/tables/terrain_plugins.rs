#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_TerrainPlugins: LazyLock<Vec<TerrainPluginsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/terrainplugins.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| TerrainPluginsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown8: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown12: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(12).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown13: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(13).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct TerrainPluginsRow {
    pub r#id: String,
    pub r#unknown8: i32,
    pub r#unknown12: bool,
    pub r#unknown13: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct TerrainPluginsRef(pub usize);

impl Deref for TerrainPluginsRef {
    type Target = TerrainPluginsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_TerrainPlugins[self.0]
    }
}

impl TerrainPluginsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static TerrainPluginsRow {
        &TABLE_TerrainPlugins[self.0]
    }
    pub fn get(&self) -> &'static TerrainPluginsRow {
        &TABLE_TerrainPlugins[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_TerrainPlugins
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static TerrainPluginsRow)> {
        TABLE_TerrainPlugins
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
        for row in TABLE_TerrainPlugins.iter() {
            black_box(row);
        }
    }
}
