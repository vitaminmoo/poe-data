#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MapStashSpecialTypeEntries: LazyLock<Vec<MapStashSpecialTypeEntriesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/mapstashspecialtypeentries.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| MapStashSpecialTypeEntriesRow {
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
            r#map_item: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(12..12 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(28..28 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown36: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(36..36 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#dds_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(40..40 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#dds_file_new: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(48..48 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#is_shaper_guardian: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(56).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#is_elder_guardian: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(57).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MapStashSpecialTypeEntriesRow {
    pub r#id: String,
    pub r#unknown8: i32,
    pub r#map_item: BaseItemTypesRef,
    pub r#name: String,
    pub r#unknown36: i32,
    pub r#dds_file: String,
    pub r#dds_file_new: String,
    pub r#is_shaper_guardian: bool,
    pub r#is_elder_guardian: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MapStashSpecialTypeEntriesRef(pub usize);

impl Deref for MapStashSpecialTypeEntriesRef {
    type Target = MapStashSpecialTypeEntriesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MapStashSpecialTypeEntries[self.0]
    }
}

impl MapStashSpecialTypeEntriesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MapStashSpecialTypeEntriesRow {
        &TABLE_MapStashSpecialTypeEntries[self.0]
    }
    pub fn get(&self) -> &'static MapStashSpecialTypeEntriesRow {
        &TABLE_MapStashSpecialTypeEntries[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MapStashSpecialTypeEntries.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MapStashSpecialTypeEntriesRow)> {
        TABLE_MapStashSpecialTypeEntries.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_MapStashSpecialTypeEntries.iter() {
            black_box(row);
        }
    }
}
