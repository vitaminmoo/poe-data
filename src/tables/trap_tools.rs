#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_TrapTools: LazyLock<Vec<TrapToolsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/traptools.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| TrapToolsRow {
            r#base_item_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#detonation_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#throw_time: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct TrapToolsRow {
    pub r#base_item_type: BaseItemTypesRef,
    pub r#detonation_type: i32,
    pub r#throw_time: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct TrapToolsRef(pub usize);

impl Deref for TrapToolsRef {
    type Target = TrapToolsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_TrapTools[self.0]
    }
}

impl TrapToolsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static TrapToolsRow {
        &TABLE_TrapTools[self.0]
    }
    pub fn get(&self) -> &'static TrapToolsRow {
        &TABLE_TrapTools[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_TrapTools.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static TrapToolsRow)> {
        TABLE_TrapTools
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
        for row in TABLE_TrapTools.iter() {
            black_box(row);
        }
    }
}
