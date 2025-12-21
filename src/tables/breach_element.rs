#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_BreachElement: LazyLock<Vec<BreachElementRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/breachelement.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| BreachElementRow {
            r#element: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown8: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#base_breachstone: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#boss_map_mod: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#duplicate_boss: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(56..56 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct BreachElementRow {
    pub r#element: String,
    pub r#unknown8: i64,
    pub r#base_breachstone: BaseItemTypesRef,
    pub r#boss_map_mod: StatsRef,
    pub r#duplicate_boss: StatsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BreachElementRef(pub usize);

impl Deref for BreachElementRef {
    type Target = BreachElementRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_BreachElement[self.0]
    }
}

impl BreachElementRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BreachElementRow {
        &TABLE_BreachElement[self.0]
    }
    pub fn get(&self) -> &'static BreachElementRow {
        &TABLE_BreachElement[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BreachElement.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BreachElementRow)> {
        TABLE_BreachElement
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
        for row in TABLE_BreachElement.iter() {
            black_box(row);
        }
    }
}
