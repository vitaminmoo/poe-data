#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_WorldDescriptionButton: LazyLock<Vec<WorldDescriptionButtonRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/worlddescriptionbutton.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| WorldDescriptionButtonRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#icon_default: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#icon_light: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown24: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(24).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#icon_wasd: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(25..25 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#sound1: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(33..33 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                SoundEffectsRef::new(value as usize)
            },
            r#sound2: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(49..49 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                SoundEffectsRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct WorldDescriptionButtonRow {
    pub r#id: String,
    pub r#icon_default: String,
    pub r#icon_light: String,
    pub r#unknown24: bool,
    pub r#icon_wasd: String,
    pub r#sound1: SoundEffectsRef,
    pub r#sound2: SoundEffectsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct WorldDescriptionButtonRef(pub usize);

impl Deref for WorldDescriptionButtonRef {
    type Target = WorldDescriptionButtonRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_WorldDescriptionButton[self.0]
    }
}

impl WorldDescriptionButtonRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static WorldDescriptionButtonRow {
        &TABLE_WorldDescriptionButton[self.0]
    }
    pub fn get(&self) -> &'static WorldDescriptionButtonRow {
        &TABLE_WorldDescriptionButton[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_WorldDescriptionButton.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static WorldDescriptionButtonRow)> {
        TABLE_WorldDescriptionButton.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_WorldDescriptionButton.iter() {
            black_box(row);
        }
    }
}
