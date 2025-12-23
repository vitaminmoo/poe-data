#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_Incursion2Medallions: LazyLock<Vec<Incursion2MedallionsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/incursion2medallions.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| Incursion2MedallionsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#flavour_text: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#icon_dds_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(24..24 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown32: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(32).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#misc_object: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(33..33 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MiscObjectsRef::new(value as usize)
            },
            r#description: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(49..49 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#sound_effect: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(57..57 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                SoundEffectsRef::new(value as usize)
            },
            r#unknown73: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(73..73 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct Incursion2MedallionsRow {
    pub r#id: String,
    pub r#name: String,
    pub r#flavour_text: String,
    pub r#icon_dds_file: String,
    pub r#unknown32: bool,
    pub r#misc_object: MiscObjectsRef,
    pub r#description: String,
    pub r#sound_effect: SoundEffectsRef,
    pub r#unknown73: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct Incursion2MedallionsRef(pub usize);

impl Deref for Incursion2MedallionsRef {
    type Target = Incursion2MedallionsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_Incursion2Medallions[self.0]
    }
}

impl Incursion2MedallionsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static Incursion2MedallionsRow {
        &TABLE_Incursion2Medallions[self.0]
    }
    pub fn get(&self) -> &'static Incursion2MedallionsRow {
        &TABLE_Incursion2Medallions[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Incursion2Medallions.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static Incursion2MedallionsRow)> {
        TABLE_Incursion2Medallions.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_Incursion2Medallions.iter() {
            black_box(row);
        }
    }
}
