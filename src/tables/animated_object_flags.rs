#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_AnimatedObjectFlags: LazyLock<Vec<AnimatedObjectFlagsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/animatedobjectflags.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| AnimatedObjectFlagsRow {
            r#ao_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown8: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown12: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(12).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown13: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(13).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct AnimatedObjectFlagsRow {
    pub r#ao_file: String,
    pub r#unknown8: i32,
    pub r#unknown12: bool,
    pub r#unknown13: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct AnimatedObjectFlagsRef(pub usize);

impl Deref for AnimatedObjectFlagsRef {
    type Target = AnimatedObjectFlagsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_AnimatedObjectFlags[self.0]
    }
}

impl AnimatedObjectFlagsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static AnimatedObjectFlagsRow {
        &TABLE_AnimatedObjectFlags[self.0]
    }
    pub fn get(&self) -> &'static AnimatedObjectFlagsRow {
        &TABLE_AnimatedObjectFlags[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_AnimatedObjectFlags
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static AnimatedObjectFlagsRow)> {
        TABLE_AnimatedObjectFlags
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
        for row in TABLE_AnimatedObjectFlags.iter() {
            black_box(row);
        }
    }
}
