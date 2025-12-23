#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_IncursionUniqueUpgradeComponents: LazyLock<Vec<IncursionUniqueUpgradeComponentsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/incursionuniqueupgradecomponents.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| IncursionUniqueUpgradeComponentsRow {
            r#base_unique: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                WordsRef::new(value as usize)
            },
            r#upgrade_currency: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct IncursionUniqueUpgradeComponentsRow {
    pub r#base_unique: WordsRef,
    pub r#upgrade_currency: BaseItemTypesRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct IncursionUniqueUpgradeComponentsRef(pub usize);

impl Deref for IncursionUniqueUpgradeComponentsRef {
    type Target = IncursionUniqueUpgradeComponentsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_IncursionUniqueUpgradeComponents[self.0]
    }
}

impl IncursionUniqueUpgradeComponentsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static IncursionUniqueUpgradeComponentsRow {
        &TABLE_IncursionUniqueUpgradeComponents[self.0]
    }
    pub fn get(&self) -> &'static IncursionUniqueUpgradeComponentsRow {
        &TABLE_IncursionUniqueUpgradeComponents[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_IncursionUniqueUpgradeComponents.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static IncursionUniqueUpgradeComponentsRow)> {
        TABLE_IncursionUniqueUpgradeComponents.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_IncursionUniqueUpgradeComponents.iter() {
            black_box(row);
        }
    }
}
