#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_RaceTimes: LazyLock<Vec<RaceTimesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/racetimes.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| RaceTimesRow {
            r#races_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                RacesRef::new(value as usize)
            },
            r#index: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#start_unix_time: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#end_unix_time: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct RaceTimesRow {
    pub r#races_key: RacesRef,
    pub r#index: i32,
    pub r#start_unix_time: i32,
    pub r#end_unix_time: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct RaceTimesRef(pub usize);

impl Deref for RaceTimesRef {
    type Target = RaceTimesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_RaceTimes[self.0]
    }
}

impl RaceTimesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static RaceTimesRow {
        &TABLE_RaceTimes[self.0]
    }
    pub fn get(&self) -> &'static RaceTimesRow {
        &TABLE_RaceTimes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_RaceTimes.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static RaceTimesRow)> {
        TABLE_RaceTimes
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
        for row in TABLE_RaceTimes.iter() {
            black_box(row);
        }
    }
}
