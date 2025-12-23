#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_BuffVisualOrbTypes: LazyLock<Vec<BuffVisualOrbTypesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/buffvisualorbtypes.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| BuffVisualOrbTypesRow {
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
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown12: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(12..12 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown20: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown24: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown28: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(28).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#radius_stat: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(29..29 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#height_stat: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(45..45 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#unknown61: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(61..61 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
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
                let value = cell_bytes.get_f32_le();
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
                let value = cell_bytes.get_f32_le();
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
                let mut cell_bytes = row.get(79..79 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct BuffVisualOrbTypesRow {
    pub r#id: String,
    pub r#unknown8: f32,
    pub r#unknown12: f32,
    pub r#unknown16: f32,
    pub r#unknown20: f32,
    pub r#unknown24: i32,
    pub r#unknown28: bool,
    pub r#radius_stat: StatsRef,
    pub r#height_stat: StatsRef,
    pub r#unknown61: i32,
    pub r#unknown65: bool,
    pub r#unknown66: f32,
    pub r#unknown70: i32,
    pub r#unknown74: f32,
    pub r#unknown78: bool,
    pub r#unknown79: f32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BuffVisualOrbTypesRef(pub usize);

impl Deref for BuffVisualOrbTypesRef {
    type Target = BuffVisualOrbTypesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_BuffVisualOrbTypes[self.0]
    }
}

impl BuffVisualOrbTypesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BuffVisualOrbTypesRow {
        &TABLE_BuffVisualOrbTypes[self.0]
    }
    pub fn get(&self) -> &'static BuffVisualOrbTypesRow {
        &TABLE_BuffVisualOrbTypes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BuffVisualOrbTypes.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BuffVisualOrbTypesRow)> {
        TABLE_BuffVisualOrbTypes.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_BuffVisualOrbTypes.iter() {
            black_box(row);
        }
    }
}
