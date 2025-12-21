#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MonsterHeightBrackets: LazyLock<Vec<MonsterHeightBracketsRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/monsterheightbrackets.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| MonsterHeightBracketsRow {
                r#id: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(0..0 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#unknown8: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(8..8 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#buff_visuals1: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(12..12 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    BuffVisualsRef::new(value as usize)
                },
                r#buff_visuals2: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(28..28 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    BuffVisualsRef::new(value as usize)
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
                    let value = cell_bytes.get_f32_le();
                    value
                },
                r#unknown52: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(52..52 + 4).unwrap();
                    let value = cell_bytes.get_f32_le();
                    value
                },
                r#tag: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(56..56 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    TagsRef::new(value as usize)
                },
                r#unknown72: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(72..72 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown76: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(76..76 + 4).unwrap();
                    let value = cell_bytes.get_f32_le();
                    value
                },
                r#unknown80: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(80..80 + 4).unwrap();
                    let value = cell_bytes.get_f32_le();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct MonsterHeightBracketsRow {
    pub r#id: String,
    pub r#unknown8: i32,
    pub r#buff_visuals1: BuffVisualsRef,
    pub r#buff_visuals2: BuffVisualsRef,
    pub r#unknown44: i32,
    pub r#unknown48: f32,
    pub r#unknown52: f32,
    pub r#tag: TagsRef,
    pub r#unknown72: i32,
    pub r#unknown76: f32,
    pub r#unknown80: f32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MonsterHeightBracketsRef(pub usize);

impl Deref for MonsterHeightBracketsRef {
    type Target = MonsterHeightBracketsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MonsterHeightBrackets[self.0]
    }
}

impl MonsterHeightBracketsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MonsterHeightBracketsRow {
        &TABLE_MonsterHeightBrackets[self.0]
    }
    pub fn get(&self) -> &'static MonsterHeightBracketsRow {
        &TABLE_MonsterHeightBrackets[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MonsterHeightBrackets
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MonsterHeightBracketsRow)> {
        TABLE_MonsterHeightBrackets
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
        for row in TABLE_MonsterHeightBrackets.iter() {
            black_box(row);
        }
    }
}
