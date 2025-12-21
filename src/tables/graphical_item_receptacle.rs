#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_GraphicalItemReceptacle: LazyLock<Vec<GraphicalItemReceptacleRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/graphicalitemreceptacle.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| GraphicalItemReceptacleRow {
                r#id: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(0..0 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#unknown8: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(8..8 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#unknown16: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(16..16 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    ClientStringsRef::new(value as usize)
                },
                r#unknown32: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(32).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#unknown33: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(33).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#sound_effect: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(34..34 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    SoundEffectsRef::new(value as usize)
                },
                r#unknown50: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(50..50 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#unknown58: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(58..58 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    value
                },
                r#script: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(74..74 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#unknown82: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(82..82 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    ClientStringsRef::new(value as usize)
                },
                r#unknown98: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(98).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#unknown99: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(99..99 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#unknown107: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(107..107 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#unknown115: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(115).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#unknown116: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(116..116 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    value
                },
                r#unknown132: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(132..132 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#unknown140: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(140..140 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    ClientStringsRef::new(value as usize)
                },
                r#unknown156: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(156..156 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct GraphicalItemReceptacleRow {
    pub r#id: String,
    pub r#unknown8: String,
    pub r#unknown16: ClientStringsRef,
    pub r#unknown32: bool,
    pub r#unknown33: bool,
    pub r#sound_effect: SoundEffectsRef,
    pub r#unknown50: String,
    pub r#unknown58: i64,
    pub r#script: String,
    pub r#unknown82: ClientStringsRef,
    pub r#unknown98: bool,
    pub r#unknown99: String,
    pub r#unknown107: String,
    pub r#unknown115: bool,
    pub r#unknown116: i64,
    pub r#unknown132: String,
    pub r#unknown140: ClientStringsRef,
    pub r#unknown156: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct GraphicalItemReceptacleRef(pub usize);

impl Deref for GraphicalItemReceptacleRef {
    type Target = GraphicalItemReceptacleRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_GraphicalItemReceptacle[self.0]
    }
}

impl GraphicalItemReceptacleRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static GraphicalItemReceptacleRow {
        &TABLE_GraphicalItemReceptacle[self.0]
    }
    pub fn get(&self) -> &'static GraphicalItemReceptacleRow {
        &TABLE_GraphicalItemReceptacle[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_GraphicalItemReceptacle
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static GraphicalItemReceptacleRow)> {
        TABLE_GraphicalItemReceptacle
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
        for row in TABLE_GraphicalItemReceptacle.iter() {
            black_box(row);
        }
    }
}
