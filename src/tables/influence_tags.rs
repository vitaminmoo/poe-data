#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_InfluenceTags: LazyLock<Vec<InfluenceTagsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/influencetags.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| InfluenceTagsRow {
            r#item_class: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ItemClassesRef::new(value as usize)
            },
            r#influence: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                InfluenceTypes::from_repr(value as usize)
            },
            r#tag: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                TagsRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct InfluenceTagsRow {
    pub r#item_class: ItemClassesRef,
    pub r#influence: Option<InfluenceTypes>,
    pub r#tag: TagsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct InfluenceTagsRef(pub usize);

impl Deref for InfluenceTagsRef {
    type Target = InfluenceTagsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_InfluenceTags[self.0]
    }
}

impl InfluenceTagsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static InfluenceTagsRow {
        &TABLE_InfluenceTags[self.0]
    }
    pub fn get(&self) -> &'static InfluenceTagsRow {
        &TABLE_InfluenceTags[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_InfluenceTags.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static InfluenceTagsRow)> {
        TABLE_InfluenceTags
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
        for row in TABLE_InfluenceTags.iter() {
            black_box(row);
        }
    }
}
