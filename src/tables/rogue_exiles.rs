#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_RogueExiles: LazyLock<Vec<RogueExilesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/rogueexiles.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| RogueExilesRow {
            r#monster_varieties_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MonsterVarietiesRef::new(value as usize)
            },
            r#unknown16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown32: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown48: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(48..48 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown64: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(64..64 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown80: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(80..80 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown96: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(96..96 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown112: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(112..112 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown128: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(128..128 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown144: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(144..144 + 16).unwrap();
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
            r#unknown160: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(160..160 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown176: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(176..176 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown192: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(192..192 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown208: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(208..208 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown224: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(224..224 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown240: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(240..240 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct RogueExilesRow {
    pub r#monster_varieties_key: MonsterVarietiesRef,
    pub r#unknown16: i64,
    pub r#unknown32: i64,
    pub r#unknown48: i64,
    pub r#unknown64: i64,
    pub r#unknown80: i64,
    pub r#unknown96: i64,
    pub r#unknown112: i64,
    pub r#unknown128: i64,
    pub r#unknown144: Vec<i32>,
    pub r#unknown160: i64,
    pub r#unknown176: i64,
    pub r#unknown192: i64,
    pub r#unknown208: i64,
    pub r#unknown224: i64,
    pub r#unknown240: i64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct RogueExilesRef(pub usize);

impl Deref for RogueExilesRef {
    type Target = RogueExilesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_RogueExiles[self.0]
    }
}

impl RogueExilesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static RogueExilesRow {
        &TABLE_RogueExiles[self.0]
    }
    pub fn get(&self) -> &'static RogueExilesRow {
        &TABLE_RogueExiles[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_RogueExiles.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static RogueExilesRow)> {
        TABLE_RogueExiles
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
        for row in TABLE_RogueExiles.iter() {
            black_box(row);
        }
    }
}
