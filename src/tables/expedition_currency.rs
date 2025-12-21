#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ExpeditionCurrency: LazyLock<Vec<ExpeditionCurrencyRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/expeditioncurrency.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ExpeditionCurrencyRow {
            r#base_item_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#npc: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ExpeditionNPCsRef::new(value as usize)
            },
            r#loot_sound: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                SoundEffectsRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ExpeditionCurrencyRow {
    pub r#base_item_type: BaseItemTypesRef,
    pub r#npc: ExpeditionNPCsRef,
    pub r#loot_sound: SoundEffectsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ExpeditionCurrencyRef(pub usize);

impl Deref for ExpeditionCurrencyRef {
    type Target = ExpeditionCurrencyRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ExpeditionCurrency[self.0]
    }
}

impl ExpeditionCurrencyRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ExpeditionCurrencyRow {
        &TABLE_ExpeditionCurrency[self.0]
    }
    pub fn get(&self) -> &'static ExpeditionCurrencyRow {
        &TABLE_ExpeditionCurrency[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ExpeditionCurrency
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ExpeditionCurrencyRow)> {
        TABLE_ExpeditionCurrency
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
        for row in TABLE_ExpeditionCurrency.iter() {
            black_box(row);
        }
    }
}
