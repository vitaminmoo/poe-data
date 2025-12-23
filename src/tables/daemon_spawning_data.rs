#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_DaemonSpawningData: LazyLock<Vec<DaemonSpawningDataRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/daemonspawningdata.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| DaemonSpawningDataRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#monster_varieties: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| MonsterVarietiesRef::new(value as usize)).collect()
            },
            r#unknown24: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown28: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(28).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown29: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(29..29 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown33: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(33..33 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown37: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(37).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown38: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(38).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown39: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(39).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct DaemonSpawningDataRow {
    pub r#id: String,
    pub r#monster_varieties: Vec<MonsterVarietiesRef>,
    pub r#unknown24: i32,
    pub r#unknown28: bool,
    pub r#unknown29: i32,
    pub r#unknown33: i32,
    pub r#unknown37: bool,
    pub r#unknown38: bool,
    pub r#unknown39: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct DaemonSpawningDataRef(pub usize);

impl Deref for DaemonSpawningDataRef {
    type Target = DaemonSpawningDataRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_DaemonSpawningData[self.0]
    }
}

impl DaemonSpawningDataRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static DaemonSpawningDataRow {
        &TABLE_DaemonSpawningData[self.0]
    }
    pub fn get(&self) -> &'static DaemonSpawningDataRow {
        &TABLE_DaemonSpawningData[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_DaemonSpawningData.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static DaemonSpawningDataRow)> {
        TABLE_DaemonSpawningData.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_DaemonSpawningData.iter() {
            black_box(row);
        }
    }
}
