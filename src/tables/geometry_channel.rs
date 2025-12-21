#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_GeometryChannel: LazyLock<Vec<GeometryChannelRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/geometrychannel.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| GeometryChannelRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown8: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown24: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown40: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown56: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(56..56 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown64: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(64..64 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown72: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(72..72 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown80: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(80).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown81: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(81).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown82: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(82..82 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown98: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(98..98 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#epk_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(114..114 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown122: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(122..122 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown126: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(126..126 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct GeometryChannelRow {
    pub r#id: String,
    pub r#unknown8: i64,
    pub r#unknown24: i64,
    pub r#unknown40: i64,
    pub r#unknown56: String,
    pub r#unknown64: String,
    pub r#unknown72: String,
    pub r#unknown80: bool,
    pub r#unknown81: bool,
    pub r#unknown82: i64,
    pub r#unknown98: i64,
    pub r#epk_file: String,
    pub r#unknown122: i32,
    pub r#unknown126: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct GeometryChannelRef(pub usize);

impl Deref for GeometryChannelRef {
    type Target = GeometryChannelRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_GeometryChannel[self.0]
    }
}

impl GeometryChannelRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static GeometryChannelRow {
        &TABLE_GeometryChannel[self.0]
    }
    pub fn get(&self) -> &'static GeometryChannelRow {
        &TABLE_GeometryChannel[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_GeometryChannel
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static GeometryChannelRow)> {
        TABLE_GeometryChannel
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
        for row in TABLE_GeometryChannel.iter() {
            black_box(row);
        }
    }
}
