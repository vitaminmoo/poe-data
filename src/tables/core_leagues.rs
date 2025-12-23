#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_CoreLeagues: LazyLock<Vec<CoreLeaguesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/coreleagues.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| CoreLeaguesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown8: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(8).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown9: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(9).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown10: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(10..10 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#unknown26: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(26..26 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| StatsRef::new(value as usize)).collect()
            },
            r#unknown42: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(42..42 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#unknown58: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(58..58 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#unknown74: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(74..74 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#unknown90: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(90..90 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| StatsRef::new(value as usize)).collect()
            },
            r#unknown106: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(106..106 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#unknown122: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(122..122 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown138: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(138..138 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown142: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(142).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown143: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(143..143 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#unknown159: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(159).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown160: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(160).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown161: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(161..161 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| StatsRef::new(value as usize)).collect()
            },
            r#unknown177: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(177).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct CoreLeaguesRow {
    pub r#id: String,
    pub r#unknown8: bool,
    pub r#unknown9: bool,
    pub r#unknown10: StatsRef,
    pub r#unknown26: Vec<StatsRef>,
    pub r#unknown42: StatsRef,
    pub r#unknown58: StatsRef,
    pub r#unknown74: StatsRef,
    pub r#unknown90: Vec<StatsRef>,
    pub r#unknown106: StatsRef,
    pub r#unknown122: i64,
    pub r#unknown138: i32,
    pub r#unknown142: bool,
    pub r#unknown143: StatsRef,
    pub r#unknown159: bool,
    pub r#unknown160: bool,
    pub r#unknown161: Vec<StatsRef>,
    pub r#unknown177: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct CoreLeaguesRef(pub usize);

impl Deref for CoreLeaguesRef {
    type Target = CoreLeaguesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_CoreLeagues[self.0]
    }
}

impl CoreLeaguesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static CoreLeaguesRow {
        &TABLE_CoreLeagues[self.0]
    }
    pub fn get(&self) -> &'static CoreLeaguesRow {
        &TABLE_CoreLeagues[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_CoreLeagues.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static CoreLeaguesRow)> {
        TABLE_CoreLeagues.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_CoreLeagues.iter() {
            black_box(row);
        }
    }
}
