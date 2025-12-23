#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MonsterMortar: LazyLock<Vec<MonsterMortarRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/monstermortar.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| MonsterMortarRow {
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
                let mut cell_bytes = row.get(52..52 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown56: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(56).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown57: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(57).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown58: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(58).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown59: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(59).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown60: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(60..60 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown64: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(64..64 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown68: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(68..68 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown72: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(72).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown73: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(73..73 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown77: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(77).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown78: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(78).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown79: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(79..79 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown95: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(95..95 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown99: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(99..99 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown103: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(103..103 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown107: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(107..107 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown111: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(111).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown112: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(112..112 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown128: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(128..128 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MonsterMortarRow {
    pub r#id: i32,
    pub r#unknown4: i64,
    pub r#unknown20: i64,
    pub r#unknown36: i64,
    pub r#unknown52: i32,
    pub r#unknown56: bool,
    pub r#unknown57: bool,
    pub r#unknown58: bool,
    pub r#unknown59: bool,
    pub r#unknown60: i32,
    pub r#unknown64: i32,
    pub r#unknown68: i32,
    pub r#unknown72: bool,
    pub r#unknown73: i32,
    pub r#unknown77: bool,
    pub r#unknown78: bool,
    pub r#unknown79: i64,
    pub r#unknown95: i32,
    pub r#unknown99: f32,
    pub r#unknown103: f32,
    pub r#unknown107: f32,
    pub r#unknown111: bool,
    pub r#unknown112: i64,
    pub r#unknown128: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MonsterMortarRef(pub usize);

impl Deref for MonsterMortarRef {
    type Target = MonsterMortarRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MonsterMortar[self.0]
    }
}

impl MonsterMortarRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MonsterMortarRow {
        &TABLE_MonsterMortar[self.0]
    }
    pub fn get(&self) -> &'static MonsterMortarRow {
        &TABLE_MonsterMortar[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MonsterMortar.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MonsterMortarRow)> {
        TABLE_MonsterMortar.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_MonsterMortar.iter() {
            black_box(row);
        }
    }
}
