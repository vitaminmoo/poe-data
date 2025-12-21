#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_HarvestCraftOptions: LazyLock<Vec<HarvestCraftOptionsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/harvestcraftoptions.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| HarvestCraftOptionsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#text: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#tier: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                HarvestCraftTiersRef::new(value as usize)
            },
            r#command: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(32..32 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#parameters: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(40..40 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown48: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(48..48 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#hash16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(64..64 + 2).unwrap();
                let value = cell_bytes.get_i16_le();
                value
            },
            r#description: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(66..66 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#is_enchant: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(74).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#lifeforce_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(75..75 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#lifeforce_cost: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(79..79 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#sacred_cost: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(83..83 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown87: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(87).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#achievements: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(88..88 + 16).unwrap();
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
            r#unknown104: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(104..104 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct HarvestCraftOptionsRow {
    pub r#id: String,
    pub r#text: String,
    pub r#tier: HarvestCraftTiersRef,
    pub r#command: String,
    pub r#parameters: String,
    pub r#unknown48: (usize, usize),
    pub r#hash16: i16,
    pub r#description: String,
    pub r#is_enchant: bool,
    pub r#lifeforce_type: i32,
    pub r#lifeforce_cost: i32,
    pub r#sacred_cost: i32,
    pub r#unknown87: bool,
    pub r#achievements: Vec<AchievementItemsRef>,
    pub r#unknown104: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct HarvestCraftOptionsRef(pub usize);

impl Deref for HarvestCraftOptionsRef {
    type Target = HarvestCraftOptionsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_HarvestCraftOptions[self.0]
    }
}

impl HarvestCraftOptionsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static HarvestCraftOptionsRow {
        &TABLE_HarvestCraftOptions[self.0]
    }
    pub fn get(&self) -> &'static HarvestCraftOptionsRow {
        &TABLE_HarvestCraftOptions[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_HarvestCraftOptions
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static HarvestCraftOptionsRow)> {
        TABLE_HarvestCraftOptions
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
        for row in TABLE_HarvestCraftOptions.iter() {
            black_box(row);
        }
    }
}
