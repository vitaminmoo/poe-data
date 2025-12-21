#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ProjectileOverrides: LazyLock<Vec<ProjectileOverridesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/projectileoverrides.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ProjectileOverridesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown8: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown24: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(24..24 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown32: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#item_visual_identity: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(48..48 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ItemVisualIdentityRef::new(value as usize)
            },
            r#unknown64: {
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
pub struct ProjectileOverridesRow {
    pub r#id: String,
    pub r#unknown8: Vec<String>,
    pub r#unknown24: String,
    pub r#unknown32: Vec<String>,
    pub r#item_visual_identity: ItemVisualIdentityRef,
    pub r#unknown64: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ProjectileOverridesRef(pub usize);

impl Deref for ProjectileOverridesRef {
    type Target = ProjectileOverridesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ProjectileOverrides[self.0]
    }
}

impl ProjectileOverridesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ProjectileOverridesRow {
        &TABLE_ProjectileOverrides[self.0]
    }
    pub fn get(&self) -> &'static ProjectileOverridesRow {
        &TABLE_ProjectileOverrides[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ProjectileOverrides
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ProjectileOverridesRow)> {
        TABLE_ProjectileOverrides
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
        for row in TABLE_ProjectileOverrides.iter() {
            black_box(row);
        }
    }
}
