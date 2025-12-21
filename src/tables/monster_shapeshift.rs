#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MonsterShapeshift: LazyLock<Vec<MonsterShapeshiftRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/monstershapeshift.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| MonsterShapeshiftRow {
            r#id: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown4: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(4..4 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown20: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown36: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(36..36 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown52: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(52..52 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown68: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(68..68 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown84: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(84..84 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown100: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(100..100 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown116: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(116..116 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown132: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(132..132 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown148: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(148..148 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown164: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(164..164 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown172: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(172..172 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MonsterShapeshiftRow {
    pub r#id: i32,
    pub r#unknown4: i64,
    pub r#unknown20: i64,
    pub r#unknown36: i64,
    pub r#unknown52: i64,
    pub r#unknown68: Vec<String>,
    pub r#unknown84: Vec<String>,
    pub r#unknown100: Vec<String>,
    pub r#unknown116: Vec<String>,
    pub r#unknown132: i64,
    pub r#unknown148: i64,
    pub r#unknown164: String,
    pub r#unknown172: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MonsterShapeshiftRef(pub usize);

impl Deref for MonsterShapeshiftRef {
    type Target = MonsterShapeshiftRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MonsterShapeshift[self.0]
    }
}

impl MonsterShapeshiftRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MonsterShapeshiftRow {
        &TABLE_MonsterShapeshift[self.0]
    }
    pub fn get(&self) -> &'static MonsterShapeshiftRow {
        &TABLE_MonsterShapeshift[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MonsterShapeshift
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MonsterShapeshiftRow)> {
        TABLE_MonsterShapeshift
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
        for row in TABLE_MonsterShapeshift.iter() {
            black_box(row);
        }
    }
}
