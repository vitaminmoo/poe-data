#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_AtlasPrimordialBossOptions: LazyLock<Vec<AtlasPrimordialBossOptionsRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/atlasprimordialbossoptions.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| AtlasPrimordialBossOptionsRow {
                r#unknown0: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(0..0 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown4: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(4..4 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#default_icon: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(8..8 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#hover_icon: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(16..16 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#highlight_icon: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(24..24 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#empty_icon: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(32..32 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#description: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(40..40 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    ClientStringsRef::new(value as usize)
                },
                r#description_active: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(56..56 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    ClientStringsRef::new(value as usize)
                },
                r#progress_tracker: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(72..72 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#progress_tracker_fill: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(80..80 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#name: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(88..88 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#map_device_tracker_fill: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(96..96 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct AtlasPrimordialBossOptionsRow {
    pub r#unknown0: i32,
    pub r#unknown4: i32,
    pub r#default_icon: String,
    pub r#hover_icon: String,
    pub r#highlight_icon: String,
    pub r#empty_icon: String,
    pub r#description: ClientStringsRef,
    pub r#description_active: ClientStringsRef,
    pub r#progress_tracker: String,
    pub r#progress_tracker_fill: String,
    pub r#name: String,
    pub r#map_device_tracker_fill: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct AtlasPrimordialBossOptionsRef(pub usize);

impl Deref for AtlasPrimordialBossOptionsRef {
    type Target = AtlasPrimordialBossOptionsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_AtlasPrimordialBossOptions[self.0]
    }
}

impl AtlasPrimordialBossOptionsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static AtlasPrimordialBossOptionsRow {
        &TABLE_AtlasPrimordialBossOptions[self.0]
    }
    pub fn get(&self) -> &'static AtlasPrimordialBossOptionsRow {
        &TABLE_AtlasPrimordialBossOptions[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_AtlasPrimordialBossOptions
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static AtlasPrimordialBossOptionsRow)>
    {
        TABLE_AtlasPrimordialBossOptions
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
        for row in TABLE_AtlasPrimordialBossOptions.iter() {
            black_box(row);
        }
    }
}
