#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_Environments: LazyLock<Vec<EnvironmentsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/environments.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| EnvironmentsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#base_env_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#corrupted_env_files: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown32: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
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
            },
            r#unknown48: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(48..48 + 16).unwrap();
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
            },
            r#unknown64: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(64..64 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#environment_transitions_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(80..80 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                EnvironmentTransitionsRef::new(value as usize)
            },
            r#preload_group: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(96..96 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                PreloadGroupsRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct EnvironmentsRow {
    pub r#id: String,
    pub r#base_env_file: String,
    pub r#corrupted_env_files: Vec<String>,
    pub r#unknown32: Vec<i64>,
    pub r#unknown48: Vec<i64>,
    pub r#unknown64: (usize, usize),
    pub r#environment_transitions_key: EnvironmentTransitionsRef,
    pub r#preload_group: PreloadGroupsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct EnvironmentsRef(pub usize);

impl Deref for EnvironmentsRef {
    type Target = EnvironmentsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_Environments[self.0]
    }
}

impl EnvironmentsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static EnvironmentsRow {
        &TABLE_Environments[self.0]
    }
    pub fn get(&self) -> &'static EnvironmentsRow {
        &TABLE_Environments[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Environments.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static EnvironmentsRow)> {
        TABLE_Environments.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_Environments.iter() {
            black_box(row);
        }
    }
}
