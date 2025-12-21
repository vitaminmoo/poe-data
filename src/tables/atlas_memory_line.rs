#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_AtlasMemoryLine: LazyLock<Vec<AtlasMemoryLineRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/atlasmemoryline.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| AtlasMemoryLineRow {
            r#league: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#league2: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#start_point_art: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#mid_point_art: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(24..24 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#end_point_art: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(32..32 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#path_art: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(40..40 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct AtlasMemoryLineRow {
    pub r#league: String,
    pub r#league2: String,
    pub r#start_point_art: String,
    pub r#mid_point_art: String,
    pub r#end_point_art: String,
    pub r#path_art: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct AtlasMemoryLineRef(pub usize);

impl Deref for AtlasMemoryLineRef {
    type Target = AtlasMemoryLineRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_AtlasMemoryLine[self.0]
    }
}

impl AtlasMemoryLineRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static AtlasMemoryLineRow {
        &TABLE_AtlasMemoryLine[self.0]
    }
    pub fn get(&self) -> &'static AtlasMemoryLineRow {
        &TABLE_AtlasMemoryLine[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_AtlasMemoryLine
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static AtlasMemoryLineRow)> {
        TABLE_AtlasMemoryLine
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
        for row in TABLE_AtlasMemoryLine.iter() {
            black_box(row);
        }
    }
}
