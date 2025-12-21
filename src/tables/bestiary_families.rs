#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_BestiaryFamilies: LazyLock<Vec<BestiaryFamiliesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/bestiaryfamilies.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| BestiaryFamiliesRow {
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
            r#icon: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#icon_small: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(24..24 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#illustration: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(32..32 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#page_art: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(40..40 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#flavour_text: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(48..48 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown56: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(56).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#tags_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(57..57 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                TagsRef::new(value as usize)
            },
            r#unknown73: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(73..73 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#mods_keys: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(77..77 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values
                    .into_iter()
                    .map(|value| ModsRef::new(value as usize))
                    .collect()
            },
            r#currency_items_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(93..93 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                CurrencyItemsRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct BestiaryFamiliesRow {
    pub r#id: String,
    pub r#name: String,
    pub r#icon: String,
    pub r#icon_small: String,
    pub r#illustration: String,
    pub r#page_art: String,
    pub r#flavour_text: String,
    pub r#unknown56: bool,
    pub r#tags_key: TagsRef,
    pub r#unknown73: i32,
    pub r#mods_keys: Vec<ModsRef>,
    pub r#currency_items_key: CurrencyItemsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BestiaryFamiliesRef(pub usize);

impl Deref for BestiaryFamiliesRef {
    type Target = BestiaryFamiliesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_BestiaryFamilies[self.0]
    }
}

impl BestiaryFamiliesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BestiaryFamiliesRow {
        &TABLE_BestiaryFamilies[self.0]
    }
    pub fn get(&self) -> &'static BestiaryFamiliesRow {
        &TABLE_BestiaryFamilies[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BestiaryFamilies
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BestiaryFamiliesRow)> {
        TABLE_BestiaryFamilies
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
        for row in TABLE_BestiaryFamilies.iter() {
            black_box(row);
        }
    }
}
