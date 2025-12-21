#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_DelveFeatures: LazyLock<Vec<DelveFeaturesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/delvefeatures.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| DelveFeaturesRow {
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
            r#spawn_weight: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 4)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i32_le())
                    .collect::<Vec<i32>>();
                values
            },
            r#world_areas_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                WorldAreasRef::new(value as usize)
            },
            r#image: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(48..48 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#achievement_items_keys: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(56..56 + 16).unwrap();
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
                    .map(|value| AchievementItemsRef::new(value as usize))
                    .collect()
            },
            r#unknown72: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(72..72 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 4)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i32_le())
                    .collect::<Vec<i32>>();
                values
            },
            r#min_depth: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(88..88 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 4)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i32_le())
                    .collect::<Vec<i32>>();
                values
            },
            r#description: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(104..104 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown112: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(112..112 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown116: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(116..116 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 4)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i32_le())
                    .collect::<Vec<i32>>();
                values
            },
            r#unknown132: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(132..132 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 4)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i32_le())
                    .collect::<Vec<i32>>();
                values
            },
            r#unknown148: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(148..148 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 4)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i32_le())
                    .collect::<Vec<i32>>();
                values
            },
            r#unknown164: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(164..164 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct DelveFeaturesRow {
    pub r#id: String,
    pub r#name: String,
    pub r#spawn_weight: Vec<i32>,
    pub r#world_areas_key: WorldAreasRef,
    pub r#image: String,
    pub r#achievement_items_keys: Vec<AchievementItemsRef>,
    pub r#unknown72: Vec<i32>,
    pub r#min_depth: Vec<i32>,
    pub r#description: String,
    pub r#unknown112: i32,
    pub r#unknown116: Vec<i32>,
    pub r#unknown132: Vec<i32>,
    pub r#unknown148: Vec<i32>,
    pub r#unknown164: (usize, usize),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct DelveFeaturesRef(pub usize);

impl Deref for DelveFeaturesRef {
    type Target = DelveFeaturesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_DelveFeatures[self.0]
    }
}

impl DelveFeaturesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static DelveFeaturesRow {
        &TABLE_DelveFeatures[self.0]
    }
    pub fn get(&self) -> &'static DelveFeaturesRow {
        &TABLE_DelveFeatures[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_DelveFeatures.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static DelveFeaturesRow)> {
        TABLE_DelveFeatures
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
        for row in TABLE_DelveFeatures.iter() {
            black_box(row);
        }
    }
}
