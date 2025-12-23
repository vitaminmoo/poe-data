#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_VirtualStatContextFlags: LazyLock<Vec<VirtualStatContextFlagsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/virtualstatcontextflags.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| VirtualStatContextFlagsRow {
            r#id: {
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
        })
        .collect()
});

#[derive(Debug)]
pub struct VirtualStatContextFlagsRow {
    pub r#id: String,
    pub r#unknown8: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct VirtualStatContextFlagsRef(pub usize);

impl Deref for VirtualStatContextFlagsRef {
    type Target = VirtualStatContextFlagsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_VirtualStatContextFlags[self.0]
    }
}

impl VirtualStatContextFlagsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static VirtualStatContextFlagsRow {
        &TABLE_VirtualStatContextFlags[self.0]
    }
    pub fn get(&self) -> &'static VirtualStatContextFlagsRow {
        &TABLE_VirtualStatContextFlags[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_VirtualStatContextFlags.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static VirtualStatContextFlagsRow)> {
        TABLE_VirtualStatContextFlags.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_VirtualStatContextFlags.iter() {
            black_box(row);
        }
    }
}
