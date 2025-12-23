#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_TableMonsterSpawners: LazyLock<Vec<TableMonsterSpawnersRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/tablemonsterspawners.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| TableMonsterSpawnersRow {
            r#metadata: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#area_level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#spawns_monsters: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(12..12 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| MonsterVarietiesRef::new(value as usize)).collect()
            },
            r#unknown28: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(28..28 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown32: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown36: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(36..36 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown40: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown44: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(44..44 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown48: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(48..48 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown52: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(52..52 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown56: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(56..56 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown60: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(60).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown61: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(61).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown62: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(62).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown63: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(63).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown64: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(64).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown65: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(65).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown66: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(66..66 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown70: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(70..70 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown74: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(74..74 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown78: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(78..78 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown82: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(82..82 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown98: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(98).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown99: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(99).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#script1: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(100..100 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown108: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(108).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown109: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(109).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#script2: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(110..110 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown118: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(118..118 + 16).unwrap();
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
            r#unknown134: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(134..134 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown138: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(138..138 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown142: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(142..142 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown146: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(146..146 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown150: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(150..150 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown154: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(154..154 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown158: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(158..158 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown162: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(162..162 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct TableMonsterSpawnersRow {
    pub r#metadata: String,
    pub r#area_level: i32,
    pub r#spawns_monsters: Vec<MonsterVarietiesRef>,
    pub r#unknown28: i32,
    pub r#unknown32: i32,
    pub r#unknown36: i32,
    pub r#unknown40: i32,
    pub r#unknown44: i32,
    pub r#unknown48: i32,
    pub r#unknown52: i32,
    pub r#unknown56: i32,
    pub r#unknown60: bool,
    pub r#unknown61: bool,
    pub r#unknown62: bool,
    pub r#unknown63: bool,
    pub r#unknown64: bool,
    pub r#unknown65: bool,
    pub r#unknown66: i32,
    pub r#unknown70: i32,
    pub r#unknown74: i32,
    pub r#unknown78: i32,
    pub r#unknown82: i64,
    pub r#unknown98: bool,
    pub r#unknown99: bool,
    pub r#script1: String,
    pub r#unknown108: bool,
    pub r#unknown109: bool,
    pub r#script2: String,
    pub r#unknown118: Vec<i32>,
    pub r#unknown134: i32,
    pub r#unknown138: i32,
    pub r#unknown142: i32,
    pub r#unknown146: i32,
    pub r#unknown150: i32,
    pub r#unknown154: i32,
    pub r#unknown158: i32,
    pub r#unknown162: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct TableMonsterSpawnersRef(pub usize);

impl Deref for TableMonsterSpawnersRef {
    type Target = TableMonsterSpawnersRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_TableMonsterSpawners[self.0]
    }
}

impl TableMonsterSpawnersRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static TableMonsterSpawnersRow {
        &TABLE_TableMonsterSpawners[self.0]
    }
    pub fn get(&self) -> &'static TableMonsterSpawnersRow {
        &TABLE_TableMonsterSpawners[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_TableMonsterSpawners.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static TableMonsterSpawnersRow)> {
        TABLE_TableMonsterSpawners.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_TableMonsterSpawners.iter() {
            black_box(row);
        }
    }
}
