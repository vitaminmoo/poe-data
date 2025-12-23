#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_EndgameMapPins: LazyLock<Vec<EndgameMapPinsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/endgamemappins.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| EndgameMapPinsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unavailable_pin: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MiscAnimatedRef::new(value as usize)
            },
            r#available_pin: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MiscAnimatedRef::new(value as usize)
            },
            r#active_pin: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MiscAnimatedRef::new(value as usize)
            },
            r#failed_pin: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(56..56 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MiscAnimatedRef::new(value as usize)
            },
            r#complete_pin: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(72..72 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MiscAnimatedRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct EndgameMapPinsRow {
    pub r#id: String,
    pub r#unavailable_pin: MiscAnimatedRef,
    pub r#available_pin: MiscAnimatedRef,
    pub r#active_pin: MiscAnimatedRef,
    pub r#failed_pin: MiscAnimatedRef,
    pub r#complete_pin: MiscAnimatedRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct EndgameMapPinsRef(pub usize);

impl Deref for EndgameMapPinsRef {
    type Target = EndgameMapPinsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_EndgameMapPins[self.0]
    }
}

impl EndgameMapPinsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static EndgameMapPinsRow {
        &TABLE_EndgameMapPins[self.0]
    }
    pub fn get(&self) -> &'static EndgameMapPinsRow {
        &TABLE_EndgameMapPins[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_EndgameMapPins.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static EndgameMapPinsRow)> {
        TABLE_EndgameMapPins.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_EndgameMapPins.iter() {
            black_box(row);
        }
    }
}
