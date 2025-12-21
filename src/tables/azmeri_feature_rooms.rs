#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_AzmeriFeatureRooms: LazyLock<Vec<AzmeriFeatureRoomsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/azmerifeaturerooms.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| AzmeriFeatureRoomsRow {
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
            r#unknown8: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown16: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown24: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown40: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(40).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct AzmeriFeatureRoomsRow {
    pub r#unknown0: i32,
    pub r#unknown4: i32,
    pub r#unknown8: String,
    pub r#unknown16: String,
    pub r#unknown24: (usize, usize),
    pub r#unknown40: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct AzmeriFeatureRoomsRef(pub usize);

impl Deref for AzmeriFeatureRoomsRef {
    type Target = AzmeriFeatureRoomsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_AzmeriFeatureRooms[self.0]
    }
}

impl AzmeriFeatureRoomsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static AzmeriFeatureRoomsRow {
        &TABLE_AzmeriFeatureRooms[self.0]
    }
    pub fn get(&self) -> &'static AzmeriFeatureRoomsRow {
        &TABLE_AzmeriFeatureRooms[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_AzmeriFeatureRooms
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static AzmeriFeatureRoomsRow)> {
        TABLE_AzmeriFeatureRooms
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
        for row in TABLE_AzmeriFeatureRooms.iter() {
            black_box(row);
        }
    }
}
