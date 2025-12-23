#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_SkillCraftingData: LazyLock<Vec<SkillCraftingDataRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/skillcraftingdata.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| SkillCraftingDataRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#character: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                CharactersRef::new(value as usize)
            },
            r#list_background: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(24..24 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#icon: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(32..32 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#console_icon: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(40..40 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(48..48 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct SkillCraftingDataRow {
    pub r#id: String,
    pub r#character: CharactersRef,
    pub r#list_background: String,
    pub r#icon: String,
    pub r#console_icon: String,
    pub r#name: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct SkillCraftingDataRef(pub usize);

impl Deref for SkillCraftingDataRef {
    type Target = SkillCraftingDataRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_SkillCraftingData[self.0]
    }
}

impl SkillCraftingDataRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static SkillCraftingDataRow {
        &TABLE_SkillCraftingData[self.0]
    }
    pub fn get(&self) -> &'static SkillCraftingDataRow {
        &TABLE_SkillCraftingData[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_SkillCraftingData.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static SkillCraftingDataRow)> {
        TABLE_SkillCraftingData.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_SkillCraftingData.iter() {
            black_box(row);
        }
    }
}
