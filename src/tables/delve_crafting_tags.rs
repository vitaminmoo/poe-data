#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_DelveCraftingTags: LazyLock<Vec<DelveCraftingTagsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/delvecraftingtags.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| DelveCraftingTagsRow {
            r#tags_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                TagsRef::new(value as usize)
            },
            r#item_class: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct DelveCraftingTagsRow {
    pub r#tags_key: TagsRef,
    pub r#item_class: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct DelveCraftingTagsRef(pub usize);

impl Deref for DelveCraftingTagsRef {
    type Target = DelveCraftingTagsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_DelveCraftingTags[self.0]
    }
}

impl DelveCraftingTagsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static DelveCraftingTagsRow {
        &TABLE_DelveCraftingTags[self.0]
    }
    pub fn get(&self) -> &'static DelveCraftingTagsRow {
        &TABLE_DelveCraftingTags[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_DelveCraftingTags.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static DelveCraftingTagsRow)> {
        TABLE_DelveCraftingTags.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_DelveCraftingTags.iter() {
            black_box(row);
        }
    }
}
