#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_HarvestSeedItems: LazyLock<Vec<HarvestSeedItemsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/harvestseeditems.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| HarvestSeedItemsRow {
            r#id: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#base_item: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(4..4 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#drop_stat: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct HarvestSeedItemsRow {
    pub r#id: i32,
    pub r#base_item: BaseItemTypesRef,
    pub r#drop_stat: StatsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct HarvestSeedItemsRef(pub usize);

impl Deref for HarvestSeedItemsRef {
    type Target = HarvestSeedItemsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_HarvestSeedItems[self.0]
    }
}

impl HarvestSeedItemsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static HarvestSeedItemsRow {
        &TABLE_HarvestSeedItems[self.0]
    }
    pub fn get(&self) -> &'static HarvestSeedItemsRow {
        &TABLE_HarvestSeedItems[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_HarvestSeedItems
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static HarvestSeedItemsRow)> {
        TABLE_HarvestSeedItems
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
        for row in TABLE_HarvestSeedItems.iter() {
            black_box(row);
        }
    }
}
