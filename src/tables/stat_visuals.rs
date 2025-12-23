#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_StatVisuals: LazyLock<Vec<StatVisualsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/statvisuals.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| StatVisualsRow {
            r#unknown0: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#epk_files: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown32: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(32).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct StatVisualsRow {
    pub r#unknown0: i64,
    pub r#epk_files: Vec<String>,
    pub r#unknown32: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct StatVisualsRef(pub usize);

impl Deref for StatVisualsRef {
    type Target = StatVisualsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_StatVisuals[self.0]
    }
}

impl StatVisualsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static StatVisualsRow {
        &TABLE_StatVisuals[self.0]
    }
    pub fn get(&self) -> &'static StatVisualsRow {
        &TABLE_StatVisuals[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_StatVisuals.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static StatVisualsRow)> {
        TABLE_StatVisuals.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_StatVisuals.iter() {
            black_box(row);
        }
    }
}
