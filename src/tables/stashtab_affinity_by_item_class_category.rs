#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_StashtabAffinityByItemClassCategory: LazyLock<
    Vec<StashtabAffinityByItemClassCategoryRow>,
> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/stashtabaffinitybyitemclasscategory.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| StashtabAffinityByItemClassCategoryRow {
            r#item_class_category: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ItemClassCategoriesRef::new(value as usize)
            },
            r#stash_tab_affinity_id: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StashTabAffinityIdRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct StashtabAffinityByItemClassCategoryRow {
    pub r#item_class_category: ItemClassCategoriesRef,
    pub r#stash_tab_affinity_id: StashTabAffinityIdRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct StashtabAffinityByItemClassCategoryRef(pub usize);

impl Deref for StashtabAffinityByItemClassCategoryRef {
    type Target = StashtabAffinityByItemClassCategoryRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_StashtabAffinityByItemClassCategory[self.0]
    }
}

impl StashtabAffinityByItemClassCategoryRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static StashtabAffinityByItemClassCategoryRow {
        &TABLE_StashtabAffinityByItemClassCategory[self.0]
    }
    pub fn get(&self) -> &'static StashtabAffinityByItemClassCategoryRow {
        &TABLE_StashtabAffinityByItemClassCategory[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_StashtabAffinityByItemClassCategory
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs(
    ) -> impl Iterator<Item = (Self, &'static StashtabAffinityByItemClassCategoryRow)> {
        TABLE_StashtabAffinityByItemClassCategory
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
        for row in TABLE_StashtabAffinityByItemClassCategory.iter() {
            black_box(row);
        }
    }
}
