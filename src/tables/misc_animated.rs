#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MiscAnimated: LazyLock<Vec<MiscAnimatedRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/miscanimated.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| MiscAnimatedRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#ao_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#preload_groups_keys: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values
                    .into_iter()
                    .map(|value| PreloadGroupsRef::new(value as usize))
                    .collect()
            },
            r#unknown32: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown36: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(36..36 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#hash32: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MiscAnimatedRow {
    pub r#id: String,
    pub r#ao_file: String,
    pub r#preload_groups_keys: Vec<PreloadGroupsRef>,
    pub r#unknown32: i32,
    pub r#unknown36: i32,
    pub r#hash32: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MiscAnimatedRef(pub usize);

impl Deref for MiscAnimatedRef {
    type Target = MiscAnimatedRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MiscAnimated[self.0]
    }
}

impl MiscAnimatedRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MiscAnimatedRow {
        &TABLE_MiscAnimated[self.0]
    }
    pub fn get(&self) -> &'static MiscAnimatedRow {
        &TABLE_MiscAnimated[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MiscAnimated.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MiscAnimatedRow)> {
        TABLE_MiscAnimated
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
        for row in TABLE_MiscAnimated.iter() {
            black_box(row);
        }
    }
}
