#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ApplyDamageFunctions: LazyLock<Vec<ApplyDamageFunctionsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/applydamagefunctions.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ApplyDamageFunctionsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#stats_keys: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| StatsRef::new(value as usize)).collect()
            },
            r#unknown24: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(24).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ApplyDamageFunctionsRow {
    pub r#id: String,
    pub r#stats_keys: Vec<StatsRef>,
    pub r#unknown24: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ApplyDamageFunctionsRef(pub usize);

impl Deref for ApplyDamageFunctionsRef {
    type Target = ApplyDamageFunctionsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ApplyDamageFunctions[self.0]
    }
}

impl ApplyDamageFunctionsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ApplyDamageFunctionsRow {
        &TABLE_ApplyDamageFunctions[self.0]
    }
    pub fn get(&self) -> &'static ApplyDamageFunctionsRow {
        &TABLE_ApplyDamageFunctions[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ApplyDamageFunctions.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ApplyDamageFunctionsRow)> {
        TABLE_ApplyDamageFunctions.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_ApplyDamageFunctions.iter() {
            black_box(row);
        }
    }
}
