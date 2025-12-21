#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_LegacyAtlasInfluenceOutcomes: LazyLock<Vec<LegacyAtlasInfluenceOutcomesRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/legacyatlasinfluenceoutcomes.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| LegacyAtlasInfluenceOutcomesRow {
                r#id: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(0..0 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#unknown8: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(8..8 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown12: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(12..12 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown16: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(16..16 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown20: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(20..20 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown24: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(24..24 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    value
                },
                r#unknown40: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(40..40 + 16).unwrap();
                    let count = cell_bytes.get_u64_le() as usize;
                    let offset = cell_bytes.get_u64_le() as usize;
                    // array_mutator column.array == true && column.type == 'array'
                    let values = (count, offset);
                    values
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct LegacyAtlasInfluenceOutcomesRow {
    pub r#id: String,
    pub r#unknown8: i32,
    pub r#unknown12: i32,
    pub r#unknown16: i32,
    pub r#unknown20: i32,
    pub r#unknown24: i64,
    pub r#unknown40: (usize, usize),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct LegacyAtlasInfluenceOutcomesRef(pub usize);

impl Deref for LegacyAtlasInfluenceOutcomesRef {
    type Target = LegacyAtlasInfluenceOutcomesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_LegacyAtlasInfluenceOutcomes[self.0]
    }
}

impl LegacyAtlasInfluenceOutcomesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static LegacyAtlasInfluenceOutcomesRow {
        &TABLE_LegacyAtlasInfluenceOutcomes[self.0]
    }
    pub fn get(&self) -> &'static LegacyAtlasInfluenceOutcomesRow {
        &TABLE_LegacyAtlasInfluenceOutcomes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_LegacyAtlasInfluenceOutcomes
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static LegacyAtlasInfluenceOutcomesRow)>
    {
        TABLE_LegacyAtlasInfluenceOutcomes
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
        for row in TABLE_LegacyAtlasInfluenceOutcomes.iter() {
            black_box(row);
        }
    }
}
