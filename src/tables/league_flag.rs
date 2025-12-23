#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_LeagueFlag: LazyLock<Vec<LeagueFlagRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/leagueflag.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| LeagueFlagRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#image: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#is_hc: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(16).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#is_ssf: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(17).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#banner: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(18..18 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#is_ruthless: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(26).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct LeagueFlagRow {
    pub r#id: String,
    pub r#image: String,
    pub r#is_hc: bool,
    pub r#is_ssf: bool,
    pub r#banner: String,
    pub r#is_ruthless: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct LeagueFlagRef(pub usize);

impl Deref for LeagueFlagRef {
    type Target = LeagueFlagRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_LeagueFlag[self.0]
    }
}

impl LeagueFlagRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static LeagueFlagRow {
        &TABLE_LeagueFlag[self.0]
    }
    pub fn get(&self) -> &'static LeagueFlagRow {
        &TABLE_LeagueFlag[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_LeagueFlag.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static LeagueFlagRow)> {
        TABLE_LeagueFlag.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_LeagueFlag.iter() {
            black_box(row);
        }
    }
}
