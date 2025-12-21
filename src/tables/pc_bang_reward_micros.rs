#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_PCBangRewardMicros: LazyLock<Vec<PCBangRewardMicrosRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/pcbangrewardmicros.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| PCBangRewardMicrosRow {
            r#unknown0: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct PCBangRewardMicrosRow {
    pub r#unknown0: i64,
    pub r#unknown16: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct PCBangRewardMicrosRef(pub usize);

impl Deref for PCBangRewardMicrosRef {
    type Target = PCBangRewardMicrosRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_PCBangRewardMicros[self.0]
    }
}

impl PCBangRewardMicrosRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static PCBangRewardMicrosRow {
        &TABLE_PCBangRewardMicros[self.0]
    }
    pub fn get(&self) -> &'static PCBangRewardMicrosRow {
        &TABLE_PCBangRewardMicros[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_PCBangRewardMicros
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static PCBangRewardMicrosRow)> {
        TABLE_PCBangRewardMicros
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
        for row in TABLE_PCBangRewardMicros.iter() {
            black_box(row);
        }
    }
}
