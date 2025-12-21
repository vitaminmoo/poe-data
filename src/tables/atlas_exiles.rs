#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_AtlasExiles: LazyLock<Vec<AtlasExilesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/atlasexiles.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| AtlasExilesRow {
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
            r#influenced_item_incr_stat: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(12..12 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#map_icon: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(28..28 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#map_icon2: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(36..36 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct AtlasExilesRow {
    pub r#id: String,
    pub r#unknown8: i32,
    pub r#influenced_item_incr_stat: StatsRef,
    pub r#map_icon: String,
    pub r#map_icon2: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct AtlasExilesRef(pub usize);

impl Deref for AtlasExilesRef {
    type Target = AtlasExilesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_AtlasExiles[self.0]
    }
}

impl AtlasExilesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static AtlasExilesRow {
        &TABLE_AtlasExiles[self.0]
    }
    pub fn get(&self) -> &'static AtlasExilesRow {
        &TABLE_AtlasExiles[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_AtlasExiles.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static AtlasExilesRow)> {
        TABLE_AtlasExiles
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
        for row in TABLE_AtlasExiles.iter() {
            black_box(row);
        }
    }
}
