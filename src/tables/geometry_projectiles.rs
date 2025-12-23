#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_GeometryProjectiles: LazyLock<Vec<GeometryProjectilesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/geometryprojectiles.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| GeometryProjectilesRow {
            r#unknown0: {
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
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(20).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown21: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(21..21 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown25: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(25).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown26: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(26..26 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown30: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(30..30 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown34: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(34).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown35: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(35..35 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown39: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(39..39 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown43: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(43..43 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown47: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(47).unwrap();
                let value = cell_bytes.to_le() != 0;
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
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(63..63 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown79: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(79).unwrap();
                let value = cell_bytes.to_le() != 0;
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
                let mut cell_bytes = row.get(96..96 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct GeometryProjectilesRow {
    pub r#unknown0: i32,
    pub r#unknown4: i64,
    pub r#unknown20: bool,
    pub r#unknown21: i32,
    pub r#unknown25: bool,
    pub r#unknown26: i32,
    pub r#unknown30: i32,
    pub r#unknown34: bool,
    pub r#unknown35: i32,
    pub r#unknown39: i32,
    pub r#unknown43: i32,
    pub r#unknown47: bool,
    pub r#unknown48: i32,
    pub r#unknown52: i32,
    pub r#unknown56: i32,
    pub r#unknown60: bool,
    pub r#unknown61: bool,
    pub r#unknown62: bool,
    pub r#unknown63: i64,
    pub r#unknown79: bool,
    pub r#unknown80: i64,
    pub r#unknown96: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct GeometryProjectilesRef(pub usize);

impl Deref for GeometryProjectilesRef {
    type Target = GeometryProjectilesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_GeometryProjectiles[self.0]
    }
}

impl GeometryProjectilesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static GeometryProjectilesRow {
        &TABLE_GeometryProjectiles[self.0]
    }
    pub fn get(&self) -> &'static GeometryProjectilesRow {
        &TABLE_GeometryProjectiles[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_GeometryProjectiles.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static GeometryProjectilesRow)> {
        TABLE_GeometryProjectiles.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_GeometryProjectiles.iter() {
            black_box(row);
        }
    }
}
