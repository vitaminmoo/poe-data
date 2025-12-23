#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_HellscapePassiveTree: LazyLock<Vec<HellscapePassiveTreeRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/hellscapepassivetree.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| HellscapePassiveTreeRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#allocations_required: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#passives: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(12..12 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| HellscapePassivesRef::new(value as usize)).collect()
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct HellscapePassiveTreeRow {
    pub r#id: String,
    pub r#allocations_required: i32,
    pub r#passives: Vec<HellscapePassivesRef>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct HellscapePassiveTreeRef(pub usize);

impl Deref for HellscapePassiveTreeRef {
    type Target = HellscapePassiveTreeRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_HellscapePassiveTree[self.0]
    }
}

impl HellscapePassiveTreeRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static HellscapePassiveTreeRow {
        &TABLE_HellscapePassiveTree[self.0]
    }
    pub fn get(&self) -> &'static HellscapePassiveTreeRow {
        &TABLE_HellscapePassiveTree[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_HellscapePassiveTree.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static HellscapePassiveTreeRow)> {
        TABLE_HellscapePassiveTree.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_HellscapePassiveTree.iter() {
            black_box(row);
        }
    }
}
