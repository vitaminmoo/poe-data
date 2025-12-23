#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_BlightedSporeAuras: LazyLock<Vec<BlightedSporeAurasRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/blightedsporeauras.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| BlightedSporeAurasRow {
            r#buff_definitions_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BuffDefinitionsRef::new(value as usize)
            },
            r#buff_stat_values: {
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
                let mut cell_bytes = row.get(32..32 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown36: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(36..36 + 16).unwrap();
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
            r#unknown52: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(52..52 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct BlightedSporeAurasRow {
    pub r#buff_definitions_key: BuffDefinitionsRef,
    pub r#buff_stat_values: Vec<i32>,
    pub r#unknown32: i32,
    pub r#unknown36: Vec<i32>,
    pub r#unknown52: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BlightedSporeAurasRef(pub usize);

impl Deref for BlightedSporeAurasRef {
    type Target = BlightedSporeAurasRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_BlightedSporeAuras[self.0]
    }
}

impl BlightedSporeAurasRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BlightedSporeAurasRow {
        &TABLE_BlightedSporeAuras[self.0]
    }
    pub fn get(&self) -> &'static BlightedSporeAurasRow {
        &TABLE_BlightedSporeAuras[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BlightedSporeAuras.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BlightedSporeAurasRow)> {
        TABLE_BlightedSporeAuras.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_BlightedSporeAuras.iter() {
            black_box(row);
        }
    }
}
