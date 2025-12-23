#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_Realms: LazyLock<Vec<RealmsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/realms.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| RealmsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#server: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#is_enabled: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(32).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#server2: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(33..33 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#short_name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(49..49 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown57: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(57..57 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 8)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| RealmsRef::new(value as usize)).collect()
            },
            r#unknown73: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(73..73 + 8).unwrap();
                let value = cell_bytes.get_i64_le();
                RealmsRef::new(value as usize)
            },
            r#unknown81: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(81..81 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#is_gamma_realm: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(85).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#speedtest_url: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(86..86 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct RealmsRow {
    pub r#id: String,
    pub r#name: String,
    pub r#server: Vec<String>,
    pub r#is_enabled: bool,
    pub r#server2: Vec<String>,
    pub r#short_name: String,
    pub r#unknown57: Vec<RealmsRef>,
    pub r#unknown73: RealmsRef,
    pub r#unknown81: i32,
    pub r#is_gamma_realm: bool,
    pub r#speedtest_url: Vec<String>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct RealmsRef(pub usize);

impl Deref for RealmsRef {
    type Target = RealmsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_Realms[self.0]
    }
}

impl RealmsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static RealmsRow {
        &TABLE_Realms[self.0]
    }
    pub fn get(&self) -> &'static RealmsRow {
        &TABLE_Realms[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Realms.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static RealmsRow)> {
        TABLE_Realms.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_Realms.iter() {
            black_box(row);
        }
    }
}
