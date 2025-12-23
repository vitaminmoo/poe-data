#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MapConnections: LazyLock<Vec<MapConnectionsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/mapconnections.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| MapConnectionsRow {
            r#map_pins_key0: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MapPinsRef::new(value as usize)
            },
            r#map_pins_key1: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MapPinsRef::new(value as usize)
            },
            r#unknown32: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#restricted_area_text: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(48..48 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
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
                let mut cell_bytes = row.get(104..104 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown108: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(108..108 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MapConnectionsRow {
    pub r#map_pins_key0: MapPinsRef,
    pub r#map_pins_key1: MapPinsRef,
    pub r#unknown32: i64,
    pub r#restricted_area_text: String,
    pub r#unknown56: i64,
    pub r#unknown72: i64,
    pub r#unknown88: i64,
    pub r#unknown104: i32,
    pub r#unknown108: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MapConnectionsRef(pub usize);

impl Deref for MapConnectionsRef {
    type Target = MapConnectionsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MapConnections[self.0]
    }
}

impl MapConnectionsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MapConnectionsRow {
        &TABLE_MapConnections[self.0]
    }
    pub fn get(&self) -> &'static MapConnectionsRow {
        &TABLE_MapConnections[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MapConnections.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MapConnectionsRow)> {
        TABLE_MapConnections.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_MapConnections.iter() {
            black_box(row);
        }
    }
}
