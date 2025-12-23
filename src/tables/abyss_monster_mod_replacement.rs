#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_AbyssMonsterModReplacement: LazyLock<Vec<AbyssMonsterModReplacementRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/abyssmonstermodreplacement.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| AbyssMonsterModReplacementRow {
            r#mod: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ModsRef::new(value as usize)
            },
            r#mods: {
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
                values.into_iter().map(|value| ModsRef::new(value as usize)).collect()
            },
            r#unknown32: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(32).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown33: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(33).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown34: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(34..34 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 4)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i32_le())
                    .collect::<Vec<i32>>();
                values
            },
            r#unknown50: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(50..50 + 16).unwrap();
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
        })
        .collect()
});

#[derive(Debug)]
pub struct AbyssMonsterModReplacementRow {
    pub r#mod: ModsRef,
    pub r#mods: Vec<ModsRef>,
    pub r#unknown32: bool,
    pub r#unknown33: bool,
    pub r#unknown34: Vec<i32>,
    pub r#unknown50: Vec<i64>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct AbyssMonsterModReplacementRef(pub usize);

impl Deref for AbyssMonsterModReplacementRef {
    type Target = AbyssMonsterModReplacementRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_AbyssMonsterModReplacement[self.0]
    }
}

impl AbyssMonsterModReplacementRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static AbyssMonsterModReplacementRow {
        &TABLE_AbyssMonsterModReplacement[self.0]
    }
    pub fn get(&self) -> &'static AbyssMonsterModReplacementRow {
        &TABLE_AbyssMonsterModReplacement[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_AbyssMonsterModReplacement.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static AbyssMonsterModReplacementRow)> {
        TABLE_AbyssMonsterModReplacement.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_AbyssMonsterModReplacement.iter() {
            black_box(row);
        }
    }
}
