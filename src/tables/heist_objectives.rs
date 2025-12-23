#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_HeistObjectives: LazyLock<Vec<HeistObjectivesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/heistobjectives.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| HeistObjectivesRow {
            r#base_item_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#scaling: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(20..20 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct HeistObjectivesRow {
    pub r#base_item_type: BaseItemTypesRef,
    pub r#scaling: f32,
    pub r#name: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct HeistObjectivesRef(pub usize);

impl Deref for HeistObjectivesRef {
    type Target = HeistObjectivesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_HeistObjectives[self.0]
    }
}

impl HeistObjectivesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static HeistObjectivesRow {
        &TABLE_HeistObjectives[self.0]
    }
    pub fn get(&self) -> &'static HeistObjectivesRow {
        &TABLE_HeistObjectives[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_HeistObjectives.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static HeistObjectivesRow)> {
        TABLE_HeistObjectives.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_HeistObjectives.iter() {
            black_box(row);
        }
    }
}
