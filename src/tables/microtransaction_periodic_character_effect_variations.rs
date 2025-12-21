#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MicrotransactionPeriodicCharacterEffectVariations: LazyLock<
    Vec<MicrotransactionPeriodicCharacterEffectVariationsRow>,
> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/microtransactionperiodiccharactereffectvariations.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| MicrotransactionPeriodicCharacterEffectVariationsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#ao_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#misc_object: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MiscObjectsRef::new(value as usize)
            },
            r#unknown32: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MicrotransactionPeriodicCharacterEffectVariationsRow {
    pub r#id: String,
    pub r#ao_file: String,
    pub r#misc_object: MiscObjectsRef,
    pub r#unknown32: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MicrotransactionPeriodicCharacterEffectVariationsRef(pub usize);

impl Deref for MicrotransactionPeriodicCharacterEffectVariationsRef {
    type Target = MicrotransactionPeriodicCharacterEffectVariationsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MicrotransactionPeriodicCharacterEffectVariations[self.0]
    }
}

impl MicrotransactionPeriodicCharacterEffectVariationsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MicrotransactionPeriodicCharacterEffectVariationsRow {
        &TABLE_MicrotransactionPeriodicCharacterEffectVariations[self.0]
    }
    pub fn get(&self) -> &'static MicrotransactionPeriodicCharacterEffectVariationsRow {
        &TABLE_MicrotransactionPeriodicCharacterEffectVariations[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MicrotransactionPeriodicCharacterEffectVariations
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<
        Item = (
            Self,
            &'static MicrotransactionPeriodicCharacterEffectVariationsRow,
        ),
    > {
        TABLE_MicrotransactionPeriodicCharacterEffectVariations
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
        for row in TABLE_MicrotransactionPeriodicCharacterEffectVariations.iter() {
            black_box(row);
        }
    }
}
