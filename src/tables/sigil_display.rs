#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_SigilDisplay: LazyLock<Vec<SigilDisplayRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/sigildisplay.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| SigilDisplayRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#active_stats_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#inactive_stats_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#dds_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(40..40 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#inactive_art_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(48..48 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#active_art_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(56..56 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#frame_art_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(64..64 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct SigilDisplayRow {
    pub r#id: String,
    pub r#active_stats_key: StatsRef,
    pub r#inactive_stats_key: StatsRef,
    pub r#dds_file: String,
    pub r#inactive_art_file: String,
    pub r#active_art_file: String,
    pub r#frame_art_file: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct SigilDisplayRef(pub usize);

impl Deref for SigilDisplayRef {
    type Target = SigilDisplayRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_SigilDisplay[self.0]
    }
}

impl SigilDisplayRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static SigilDisplayRow {
        &TABLE_SigilDisplay[self.0]
    }
    pub fn get(&self) -> &'static SigilDisplayRow {
        &TABLE_SigilDisplay[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_SigilDisplay.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static SigilDisplayRow)> {
        TABLE_SigilDisplay
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
        for row in TABLE_SigilDisplay.iter() {
            black_box(row);
        }
    }
}
