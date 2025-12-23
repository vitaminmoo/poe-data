#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MiscBeams: LazyLock<Vec<MiscBeamsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/miscbeams.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| MiscBeamsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#misc_animated: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MiscAnimatedRef::new(value as usize)
            },
            r#unknown24: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#preload_groups_keys: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(28..28 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| PreloadGroupsRef::new(value as usize)).collect()
            },
            r#unknown44: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(44..44 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MiscBeamsRow {
    pub r#id: String,
    pub r#misc_animated: MiscAnimatedRef,
    pub r#unknown24: i32,
    pub r#preload_groups_keys: Vec<PreloadGroupsRef>,
    pub r#unknown44: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MiscBeamsRef(pub usize);

impl Deref for MiscBeamsRef {
    type Target = MiscBeamsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MiscBeams[self.0]
    }
}

impl MiscBeamsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MiscBeamsRow {
        &TABLE_MiscBeams[self.0]
    }
    pub fn get(&self) -> &'static MiscBeamsRow {
        &TABLE_MiscBeams[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MiscBeams.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MiscBeamsRow)> {
        TABLE_MiscBeams.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_MiscBeams.iter() {
            black_box(row);
        }
    }
}
