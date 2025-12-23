#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_SkillGemsForUniqueStat: LazyLock<Vec<SkillGemsForUniqueStatRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/skillgemsforuniquestat.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| SkillGemsForUniqueStatRow {
            r#index: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#skill_gems: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(4..4 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| SkillGemsRef::new(value as usize)).collect()
            },
            r#unknown20: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(20).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown21: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(21..21 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct SkillGemsForUniqueStatRow {
    pub r#index: i32,
    pub r#skill_gems: Vec<SkillGemsRef>,
    pub r#unknown20: bool,
    pub r#unknown21: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct SkillGemsForUniqueStatRef(pub usize);

impl Deref for SkillGemsForUniqueStatRef {
    type Target = SkillGemsForUniqueStatRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_SkillGemsForUniqueStat[self.0]
    }
}

impl SkillGemsForUniqueStatRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static SkillGemsForUniqueStatRow {
        &TABLE_SkillGemsForUniqueStat[self.0]
    }
    pub fn get(&self) -> &'static SkillGemsForUniqueStatRow {
        &TABLE_SkillGemsForUniqueStat[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_SkillGemsForUniqueStat.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static SkillGemsForUniqueStatRow)> {
        TABLE_SkillGemsForUniqueStat.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_SkillGemsForUniqueStat.iter() {
            black_box(row);
        }
    }
}
