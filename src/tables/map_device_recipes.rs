#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MapDeviceRecipes: LazyLock<Vec<MapDeviceRecipesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/mapdevicerecipes.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| MapDeviceRecipesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#recipe_items: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
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
                    .map(|value| BaseItemTypesRef::new(value as usize))
                    .collect()
            },
            r#unknown24: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown40: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown56: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(56..56 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown60: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(60..60 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown76: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(76..76 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown80: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(80).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown81: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(81).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown82: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(82).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown83: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(83..83 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown99: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(99).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown100: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(100..100 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown116: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(116).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown117: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(117).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MapDeviceRecipesRow {
    pub r#id: String,
    pub r#recipe_items: Vec<BaseItemTypesRef>,
    pub r#unknown24: (usize, usize),
    pub r#unknown40: i64,
    pub r#unknown56: i32,
    pub r#unknown60: i64,
    pub r#unknown76: i32,
    pub r#unknown80: bool,
    pub r#unknown81: bool,
    pub r#unknown82: bool,
    pub r#unknown83: (usize, usize),
    pub r#unknown99: bool,
    pub r#unknown100: (usize, usize),
    pub r#unknown116: bool,
    pub r#unknown117: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MapDeviceRecipesRef(pub usize);

impl Deref for MapDeviceRecipesRef {
    type Target = MapDeviceRecipesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MapDeviceRecipes[self.0]
    }
}

impl MapDeviceRecipesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MapDeviceRecipesRow {
        &TABLE_MapDeviceRecipes[self.0]
    }
    pub fn get(&self) -> &'static MapDeviceRecipesRow {
        &TABLE_MapDeviceRecipes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MapDeviceRecipes
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MapDeviceRecipesRow)> {
        TABLE_MapDeviceRecipes
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
        for row in TABLE_MapDeviceRecipes.iter() {
            black_box(row);
        }
    }
}
