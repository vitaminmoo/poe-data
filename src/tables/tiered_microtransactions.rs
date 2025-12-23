#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_TieredMicrotransactions: LazyLock<Vec<TieredMicrotransactionsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/tieredmicrotransactions.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| TieredMicrotransactionsRow {
            r#mtx: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#tier_thresholds: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
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
            r#unknown32: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#unknown48: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(48..48 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown64: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(64..64 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#tier_count: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(80..80 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown84: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(84..84 + 16).unwrap();
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
            r#unknown100: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(100..100 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown116: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(116..116 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown132: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(132).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown133: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(133).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown134: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(134..134 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#unknown150: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(150).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct TieredMicrotransactionsRow {
    pub r#mtx: BaseItemTypesRef,
    pub r#tier_thresholds: Vec<i32>,
    pub r#unknown32: StatsRef,
    pub r#unknown48: (usize, usize),
    pub r#unknown64: StatsRef,
    pub r#tier_count: i32,
    pub r#unknown84: Vec<StatsRef>,
    pub r#unknown100: (usize, usize),
    pub r#unknown116: (usize, usize),
    pub r#unknown132: bool,
    pub r#unknown133: bool,
    pub r#unknown134: StatsRef,
    pub r#unknown150: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct TieredMicrotransactionsRef(pub usize);

impl Deref for TieredMicrotransactionsRef {
    type Target = TieredMicrotransactionsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_TieredMicrotransactions[self.0]
    }
}

impl TieredMicrotransactionsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static TieredMicrotransactionsRow {
        &TABLE_TieredMicrotransactions[self.0]
    }
    pub fn get(&self) -> &'static TieredMicrotransactionsRow {
        &TABLE_TieredMicrotransactions[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_TieredMicrotransactions.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static TieredMicrotransactionsRow)> {
        TABLE_TieredMicrotransactions.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_TieredMicrotransactions.iter() {
            black_box(row);
        }
    }
}
