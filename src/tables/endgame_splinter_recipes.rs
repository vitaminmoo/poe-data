#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_EndgameSplinterRecipes: LazyLock<Vec<EndgameSplinterRecipesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/endgamesplinterrecipes.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| EndgameSplinterRecipesRow {
            r#base_item_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#difficulty: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#cost: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#destination_area: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                WorldAreasRef::new(value as usize)
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
            r#unknown60: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(60..60 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown76: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(76..76 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown92: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(92..92 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct EndgameSplinterRecipesRow {
    pub r#base_item_type: BaseItemTypesRef,
    pub r#difficulty: i32,
    pub r#cost: i32,
    pub r#destination_area: WorldAreasRef,
    pub r#unknown40: i32,
    pub r#unknown44: i64,
    pub r#unknown60: (usize, usize),
    pub r#unknown76: (usize, usize),
    pub r#unknown92: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct EndgameSplinterRecipesRef(pub usize);

impl Deref for EndgameSplinterRecipesRef {
    type Target = EndgameSplinterRecipesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_EndgameSplinterRecipes[self.0]
    }
}

impl EndgameSplinterRecipesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static EndgameSplinterRecipesRow {
        &TABLE_EndgameSplinterRecipes[self.0]
    }
    pub fn get(&self) -> &'static EndgameSplinterRecipesRow {
        &TABLE_EndgameSplinterRecipes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_EndgameSplinterRecipes.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static EndgameSplinterRecipesRow)> {
        TABLE_EndgameSplinterRecipes.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_EndgameSplinterRecipes.iter() {
            black_box(row);
        }
    }
}
