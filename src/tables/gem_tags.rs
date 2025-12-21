#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_GemTags: LazyLock<Vec<GemTagsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/gemtags.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| GemTagsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#local_level_stat: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#local_quality_stat: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#global_spell_level_stat: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(48..48 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct GemTagsRow {
    pub r#id: String,
    pub r#name: String,
    pub r#local_level_stat: StatsRef,
    pub r#local_quality_stat: StatsRef,
    pub r#global_spell_level_stat: StatsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct GemTagsRef(pub usize);

impl Deref for GemTagsRef {
    type Target = GemTagsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_GemTags[self.0]
    }
}

impl GemTagsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static GemTagsRow {
        &TABLE_GemTags[self.0]
    }
    pub fn get(&self) -> &'static GemTagsRow {
        &TABLE_GemTags[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_GemTags.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static GemTagsRow)> {
        TABLE_GemTags.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_GemTags.iter() {
            black_box(row);
        }
    }
}
