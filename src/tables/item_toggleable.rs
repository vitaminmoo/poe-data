#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ItemToggleable: LazyLock<Vec<ItemToggleableRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/itemtoggleable.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ItemToggleableRow {
            r#base_item_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ItemToggleableRow {
    pub r#base_item_type: BaseItemTypesRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ItemToggleableRef(pub usize);

impl Deref for ItemToggleableRef {
    type Target = ItemToggleableRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ItemToggleable[self.0]
    }
}

impl ItemToggleableRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ItemToggleableRow {
        &TABLE_ItemToggleable[self.0]
    }
    pub fn get(&self) -> &'static ItemToggleableRow {
        &TABLE_ItemToggleable[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ItemToggleable
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ItemToggleableRow)> {
        TABLE_ItemToggleable
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
        for row in TABLE_ItemToggleable.iter() {
            black_box(row);
        }
    }
}
