#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ModEffectStats: LazyLock<Vec<ModEffectStatsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/modeffectstats.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| ModEffectStatsRow {
            r#stat: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#tags: {
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
                values.into_iter().map(|value| TagsRef::new(value as usize)).collect()
            },
            r#unknown32: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(32).unwrap();
                let value = cell_bytes.to_le() != 0;
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
        })
        .collect()
});

#[derive(Debug)]
pub struct ModEffectStatsRow {
    pub r#stat: StatsRef,
    pub r#tags: Vec<TagsRef>,
    pub r#unknown32: bool,
    pub r#unknown33: i32,
    pub r#unknown37: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ModEffectStatsRef(pub usize);

impl Deref for ModEffectStatsRef {
    type Target = ModEffectStatsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ModEffectStats[self.0]
    }
}

impl ModEffectStatsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ModEffectStatsRow {
        &TABLE_ModEffectStats[self.0]
    }
    pub fn get(&self) -> &'static ModEffectStatsRow {
        &TABLE_ModEffectStats[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ModEffectStats.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ModEffectStatsRow)> {
        TABLE_ModEffectStats.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_ModEffectStats.iter() {
            black_box(row);
        }
    }
}
