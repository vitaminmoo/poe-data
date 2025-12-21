#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ExpeditionDeals: LazyLock<Vec<ExpeditionDealsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/expeditiondeals.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ExpeditionDealsRow {
            r#id: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#function: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(4..4 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#arguments: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(12..12 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown20: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#description: {
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
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown48: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(48..48 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown56: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(56..56 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown60: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(60..60 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#deal_family: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(64..64 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                ExpeditionDealFamilies::from_repr(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ExpeditionDealsRow {
    pub r#id: i32,
    pub r#function: String,
    pub r#arguments: String,
    pub r#unknown20: i32,
    pub r#description: String,
    pub r#unknown32: (usize, usize),
    pub r#unknown48: String,
    pub r#unknown56: i32,
    pub r#unknown60: i32,
    pub r#deal_family: Option<ExpeditionDealFamilies>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ExpeditionDealsRef(pub usize);

impl Deref for ExpeditionDealsRef {
    type Target = ExpeditionDealsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ExpeditionDeals[self.0]
    }
}

impl ExpeditionDealsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ExpeditionDealsRow {
        &TABLE_ExpeditionDeals[self.0]
    }
    pub fn get(&self) -> &'static ExpeditionDealsRow {
        &TABLE_ExpeditionDeals[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ExpeditionDeals
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ExpeditionDealsRow)> {
        TABLE_ExpeditionDeals
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
        for row in TABLE_ExpeditionDeals.iter() {
            black_box(row);
        }
    }
}
