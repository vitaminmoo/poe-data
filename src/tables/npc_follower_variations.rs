#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_NPCFollowerVariations: LazyLock<Vec<NPCFollowerVariationsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/npcfollowervariations.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| NPCFollowerVariationsRow {
            r#monster_varieties_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MonsterVarietiesRef::new(value as usize)
            },
            r#misc_animated_key0: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MiscAnimatedRef::new(value as usize)
            },
            r#misc_animated_key1: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MiscAnimatedRef::new(value as usize)
            },
            r#unknown48: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(48).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown49: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(49).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown50: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(50..50 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown54: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(54..54 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown58: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(58..58 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown62: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(62..62 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown66: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(66).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown67: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(67).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown68: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(68..68 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown84: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(84..84 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown100: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(100..100 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown104: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(104).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown105: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(105).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown106: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(106..106 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct NPCFollowerVariationsRow {
    pub r#monster_varieties_key: MonsterVarietiesRef,
    pub r#misc_animated_key0: MiscAnimatedRef,
    pub r#misc_animated_key1: MiscAnimatedRef,
    pub r#unknown48: bool,
    pub r#unknown49: bool,
    pub r#unknown50: i32,
    pub r#unknown54: i32,
    pub r#unknown58: i32,
    pub r#unknown62: i32,
    pub r#unknown66: bool,
    pub r#unknown67: bool,
    pub r#unknown68: (usize, usize),
    pub r#unknown84: (usize, usize),
    pub r#unknown100: i32,
    pub r#unknown104: bool,
    pub r#unknown105: bool,
    pub r#unknown106: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct NPCFollowerVariationsRef(pub usize);

impl Deref for NPCFollowerVariationsRef {
    type Target = NPCFollowerVariationsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_NPCFollowerVariations[self.0]
    }
}

impl NPCFollowerVariationsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static NPCFollowerVariationsRow {
        &TABLE_NPCFollowerVariations[self.0]
    }
    pub fn get(&self) -> &'static NPCFollowerVariationsRow {
        &TABLE_NPCFollowerVariations[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_NPCFollowerVariations.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static NPCFollowerVariationsRow)> {
        TABLE_NPCFollowerVariations.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_NPCFollowerVariations.iter() {
            black_box(row);
        }
    }
}
