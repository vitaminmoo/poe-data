#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_StashTabAffinityByBaseItemType: LazyLock<Vec<StashTabAffinityByBaseItemTypeRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/stashtabaffinitybybaseitemtype.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| StashTabAffinityByBaseItemTypeRow {
            r#base_item_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#stash_tab_affinity_id: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| StashTabAffinityIdRef::new(value as usize)).collect()
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct StashTabAffinityByBaseItemTypeRow {
    pub r#base_item_type: BaseItemTypesRef,
    pub r#stash_tab_affinity_id: Vec<StashTabAffinityIdRef>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct StashTabAffinityByBaseItemTypeRef(pub usize);

impl Deref for StashTabAffinityByBaseItemTypeRef {
    type Target = StashTabAffinityByBaseItemTypeRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_StashTabAffinityByBaseItemType[self.0]
    }
}

impl StashTabAffinityByBaseItemTypeRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static StashTabAffinityByBaseItemTypeRow {
        &TABLE_StashTabAffinityByBaseItemType[self.0]
    }
    pub fn get(&self) -> &'static StashTabAffinityByBaseItemTypeRow {
        &TABLE_StashTabAffinityByBaseItemType[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_StashTabAffinityByBaseItemType.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static StashTabAffinityByBaseItemTypeRow)> {
        TABLE_StashTabAffinityByBaseItemType.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_StashTabAffinityByBaseItemType.iter() {
            black_box(row);
        }
    }
}
