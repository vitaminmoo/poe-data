#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_InfluenceExalts: LazyLock<Vec<InfluenceExaltsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/influenceexalts.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| InfluenceExaltsRow {
            r#influence: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                InfluenceTypes::from_repr(value as usize)
            },
            r#base_item_types_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(4..4 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct InfluenceExaltsRow {
    pub r#influence: Option<InfluenceTypes>,
    pub r#base_item_types_key: BaseItemTypesRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct InfluenceExaltsRef(pub usize);

impl Deref for InfluenceExaltsRef {
    type Target = InfluenceExaltsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_InfluenceExalts[self.0]
    }
}

impl InfluenceExaltsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static InfluenceExaltsRow {
        &TABLE_InfluenceExalts[self.0]
    }
    pub fn get(&self) -> &'static InfluenceExaltsRow {
        &TABLE_InfluenceExalts[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_InfluenceExalts
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static InfluenceExaltsRow)> {
        TABLE_InfluenceExalts
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
        for row in TABLE_InfluenceExalts.iter() {
            black_box(row);
        }
    }
}
