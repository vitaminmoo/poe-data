#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MapDevices: LazyLock<Vec<MapDevicesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/mapdevices.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| MapDevicesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#misc_object: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MiscObjectsRef::new(value as usize)
            },
            r#unknown24: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown28: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(28..28 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown36: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(36..36 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown44: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(44..44 + 16).unwrap();
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
            r#unknown60: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(60).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown61: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(61..61 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown65: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(65).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown66: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(66..66 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown70: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(70).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown71: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(71..71 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown75: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(75..75 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown79: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(79..79 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown83: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(83..83 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown87: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(87..87 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown91: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(91).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown92: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(92).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown93: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(93).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MapDevicesRow {
    pub r#id: String,
    pub r#misc_object: MiscObjectsRef,
    pub r#unknown24: i32,
    pub r#unknown28: String,
    pub r#unknown36: String,
    pub r#unknown44: Vec<i32>,
    pub r#unknown60: bool,
    pub r#unknown61: i32,
    pub r#unknown65: bool,
    pub r#unknown66: i32,
    pub r#unknown70: bool,
    pub r#unknown71: f32,
    pub r#unknown75: i32,
    pub r#unknown79: i32,
    pub r#unknown83: i32,
    pub r#unknown87: i32,
    pub r#unknown91: bool,
    pub r#unknown92: bool,
    pub r#unknown93: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MapDevicesRef(pub usize);

impl Deref for MapDevicesRef {
    type Target = MapDevicesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MapDevices[self.0]
    }
}

impl MapDevicesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MapDevicesRow {
        &TABLE_MapDevices[self.0]
    }
    pub fn get(&self) -> &'static MapDevicesRow {
        &TABLE_MapDevices[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MapDevices.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MapDevicesRow)> {
        TABLE_MapDevices.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_MapDevices.iter() {
            black_box(row);
        }
    }
}
