#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_CurrencyUseEffectsFromItem: LazyLock<Vec<CurrencyUseEffectsFromItemRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/currencyuseeffectsfromitem.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| CurrencyUseEffectsFromItemRow {
                r#base_item_type: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(0..0 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    BaseItemTypesRef::new(value as usize)
                },
                r#unknown16: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(16..16 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#effect: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(20..20 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    CurrencyUseEffectsRef::new(value as usize)
                },
                r#stat: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(36..36 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    StatsRef::new(value as usize)
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct CurrencyUseEffectsFromItemRow {
    pub r#base_item_type: BaseItemTypesRef,
    pub r#unknown16: i32,
    pub r#effect: CurrencyUseEffectsRef,
    pub r#stat: StatsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct CurrencyUseEffectsFromItemRef(pub usize);

impl Deref for CurrencyUseEffectsFromItemRef {
    type Target = CurrencyUseEffectsFromItemRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_CurrencyUseEffectsFromItem[self.0]
    }
}

impl CurrencyUseEffectsFromItemRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static CurrencyUseEffectsFromItemRow {
        &TABLE_CurrencyUseEffectsFromItem[self.0]
    }
    pub fn get(&self) -> &'static CurrencyUseEffectsFromItemRow {
        &TABLE_CurrencyUseEffectsFromItem[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_CurrencyUseEffectsFromItem
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static CurrencyUseEffectsFromItemRow)>
    {
        TABLE_CurrencyUseEffectsFromItem
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
        for row in TABLE_CurrencyUseEffectsFromItem.iter() {
            black_box(row);
        }
    }
}
