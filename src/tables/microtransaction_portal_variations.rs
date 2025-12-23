#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MicrotransactionPortalVariations: LazyLock<Vec<MicrotransactionPortalVariationsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/microtransactionportalvariations.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| MicrotransactionPortalVariationsRow {
            r#mtx_type_game_specific: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MtxTypeGameSpecificRef::new(value as usize)
            },
            r#id: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 2).unwrap();
                let value = cell_bytes.get_i16_le();
                value
            },
            r#ao_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(18..18 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#map_ao_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(26..26 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown34: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(34..34 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#misc_object: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(38..38 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MiscObjectsRef::new(value as usize)
            },
            r#portal_effect: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(54..54 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown62: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(62..62 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown66: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(66..66 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#portal_effect_large: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(74..74 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown82: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(82..82 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown86: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(86..86 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#script: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(90..90 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MicrotransactionPortalVariationsRow {
    pub r#mtx_type_game_specific: MtxTypeGameSpecificRef,
    pub r#id: i16,
    pub r#ao_file: String,
    pub r#map_ao_file: String,
    pub r#unknown34: f32,
    pub r#misc_object: MiscObjectsRef,
    pub r#portal_effect: String,
    pub r#unknown62: f32,
    pub r#unknown66: String,
    pub r#portal_effect_large: String,
    pub r#unknown82: i32,
    pub r#unknown86: i32,
    pub r#script: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MicrotransactionPortalVariationsRef(pub usize);

impl Deref for MicrotransactionPortalVariationsRef {
    type Target = MicrotransactionPortalVariationsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MicrotransactionPortalVariations[self.0]
    }
}

impl MicrotransactionPortalVariationsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MicrotransactionPortalVariationsRow {
        &TABLE_MicrotransactionPortalVariations[self.0]
    }
    pub fn get(&self) -> &'static MicrotransactionPortalVariationsRow {
        &TABLE_MicrotransactionPortalVariations[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MicrotransactionPortalVariations.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MicrotransactionPortalVariationsRow)> {
        TABLE_MicrotransactionPortalVariations.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_MicrotransactionPortalVariations.iter() {
            black_box(row);
        }
    }
}
