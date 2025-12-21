#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_SingleGroundLaser: LazyLock<Vec<SingleGroundLaserRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/singlegroundlaser.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| SingleGroundLaserRow {
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
                let mut cell_bytes = row.get(36..36 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown40: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(40..40 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown48: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(48).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown49: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(49..49 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown53: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(53).unwrap();
                let value = cell_bytes.to_le() != 0;
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
                let mut cell_bytes = row.get(62..62 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown78: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(78..78 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown94: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(94..94 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown98: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(98).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown99: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(99..99 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown103: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(103..103 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct SingleGroundLaserRow {
    pub r#id: i32,
    pub r#unknown4: i64,
    pub r#unknown20: i64,
    pub r#unknown36: i32,
    pub r#unknown40: String,
    pub r#unknown48: bool,
    pub r#unknown49: i32,
    pub r#unknown53: bool,
    pub r#unknown54: i32,
    pub r#unknown58: i32,
    pub r#unknown62: i64,
    pub r#unknown78: i64,
    pub r#unknown94: i32,
    pub r#unknown98: bool,
    pub r#unknown99: i32,
    pub r#unknown103: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct SingleGroundLaserRef(pub usize);

impl Deref for SingleGroundLaserRef {
    type Target = SingleGroundLaserRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_SingleGroundLaser[self.0]
    }
}

impl SingleGroundLaserRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static SingleGroundLaserRow {
        &TABLE_SingleGroundLaser[self.0]
    }
    pub fn get(&self) -> &'static SingleGroundLaserRow {
        &TABLE_SingleGroundLaser[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_SingleGroundLaser
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static SingleGroundLaserRow)> {
        TABLE_SingleGroundLaser
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
        for row in TABLE_SingleGroundLaser.iter() {
            black_box(row);
        }
    }
}
