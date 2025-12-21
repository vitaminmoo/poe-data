#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MicroMigrationData: LazyLock<Vec<MicroMigrationDataRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/micromigrationdata.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| MicroMigrationDataRow {
            r#base_item_types_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#unknown16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown20: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown36: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(36..36 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MicroMigrationDataRow {
    pub r#base_item_types_key: BaseItemTypesRef,
    pub r#unknown16: i32,
    pub r#unknown20: i64,
    pub r#unknown36: i64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MicroMigrationDataRef(pub usize);

impl Deref for MicroMigrationDataRef {
    type Target = MicroMigrationDataRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MicroMigrationData[self.0]
    }
}

impl MicroMigrationDataRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MicroMigrationDataRow {
        &TABLE_MicroMigrationData[self.0]
    }
    pub fn get(&self) -> &'static MicroMigrationDataRow {
        &TABLE_MicroMigrationData[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MicroMigrationData
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MicroMigrationDataRow)> {
        TABLE_MicroMigrationData
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
        for row in TABLE_MicroMigrationData.iter() {
            black_box(row);
        }
    }
}
