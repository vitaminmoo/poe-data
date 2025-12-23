#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_EndgameMaps: LazyLock<Vec<EndgameMapsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/endgamemaps.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| EndgameMapsRow {
            r#unknown0: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#world_area: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(4..4 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                WorldAreasRef::new(value as usize)
            },
            r#unknown20: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(20..20 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 4)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i32_le())
                    .collect::<Vec<i32>>();
                values
            },
            r#unknown36: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(36..36 + 16).unwrap();
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
            },
            r#flavour_text: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(52..52 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown60: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(60..60 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown64: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(64..64 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 4)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i32_le())
                    .collect::<Vec<i32>>();
                values
            },
            r#unknown80: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(80..80 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown96: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(96..96 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown112: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(112..112 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown128: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(128..128 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown144: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(144..144 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown148: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(148..148 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown152: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(152..152 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown168: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(168..168 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#map_locations: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(184..184 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| EndgameMapLocationRef::new(value as usize)).collect()
            },
            r#special_map_text: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(200..200 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#special_map_flavour_text: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(208..208 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#special_map_help_text: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(216..216 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct EndgameMapsRow {
    pub r#unknown0: i32,
    pub r#world_area: WorldAreasRef,
    pub r#unknown20: Vec<i32>,
    pub r#unknown36: Vec<i64>,
    pub r#flavour_text: String,
    pub r#unknown60: i32,
    pub r#unknown64: Vec<i32>,
    pub r#unknown80: i64,
    pub r#unknown96: i64,
    pub r#unknown112: i64,
    pub r#unknown128: i64,
    pub r#unknown144: i32,
    pub r#unknown148: i32,
    pub r#unknown152: i64,
    pub r#unknown168: i64,
    pub r#map_locations: Vec<EndgameMapLocationRef>,
    pub r#special_map_text: String,
    pub r#special_map_flavour_text: String,
    pub r#special_map_help_text: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct EndgameMapsRef(pub usize);

impl Deref for EndgameMapsRef {
    type Target = EndgameMapsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_EndgameMaps[self.0]
    }
}

impl EndgameMapsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static EndgameMapsRow {
        &TABLE_EndgameMaps[self.0]
    }
    pub fn get(&self) -> &'static EndgameMapsRow {
        &TABLE_EndgameMaps[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_EndgameMaps.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static EndgameMapsRow)> {
        TABLE_EndgameMaps.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_EndgameMaps.iter() {
            black_box(row);
        }
    }
}
