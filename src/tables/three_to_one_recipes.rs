#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ThreeToOneRecipes: LazyLock<Vec<ThreeToOneRecipesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/threetoonerecipes.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| ThreeToOneRecipesRow {
            r#unknown0: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#input_base_item: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#output_base_item: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#unknown40: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown44: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(44..44 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#upgradable_in_stashes: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(60..60 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 4)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i32_le())
                    .collect::<Vec<i32>>();
                values.into_iter().map(|value| StashId::from_repr(value as usize)).collect()
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ThreeToOneRecipesRow {
    pub r#unknown0: String,
    pub r#input_base_item: BaseItemTypesRef,
    pub r#output_base_item: BaseItemTypesRef,
    pub r#unknown40: i32,
    pub r#unknown44: i64,
    pub r#upgradable_in_stashes: Vec<Option<StashId>>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ThreeToOneRecipesRef(pub usize);

impl Deref for ThreeToOneRecipesRef {
    type Target = ThreeToOneRecipesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ThreeToOneRecipes[self.0]
    }
}

impl ThreeToOneRecipesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ThreeToOneRecipesRow {
        &TABLE_ThreeToOneRecipes[self.0]
    }
    pub fn get(&self) -> &'static ThreeToOneRecipesRow {
        &TABLE_ThreeToOneRecipes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ThreeToOneRecipes.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ThreeToOneRecipesRow)> {
        TABLE_ThreeToOneRecipes.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_ThreeToOneRecipes.iter() {
            black_box(row);
        }
    }
}
