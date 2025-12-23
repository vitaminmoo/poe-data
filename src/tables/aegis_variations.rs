#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_AegisVariations: LazyLock<Vec<AegisVariationsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/aegisvariations.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| AegisVariationsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#defends_against_physical: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(8).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#defends_against_fire: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(9).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#defends_against_cold: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(10).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#defends_against_lightning: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(11).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#defends_against_chaos: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(12).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#definition_buff: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(13..13 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BuffDefinitionsRef::new(value as usize)
            },
            r#definition_depleted: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(29..29 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BuffDefinitionsRef::new(value as usize)
            },
            r#definition_value: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(45..45 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BuffDefinitionsRef::new(value as usize)
            },
            r#unknown61: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(61).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown62: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(62..62 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown78: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(78..78 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct AegisVariationsRow {
    pub r#id: String,
    pub r#defends_against_physical: bool,
    pub r#defends_against_fire: bool,
    pub r#defends_against_cold: bool,
    pub r#defends_against_lightning: bool,
    pub r#defends_against_chaos: bool,
    pub r#definition_buff: BuffDefinitionsRef,
    pub r#definition_depleted: BuffDefinitionsRef,
    pub r#definition_value: BuffDefinitionsRef,
    pub r#unknown61: bool,
    pub r#unknown62: i64,
    pub r#unknown78: Vec<String>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct AegisVariationsRef(pub usize);

impl Deref for AegisVariationsRef {
    type Target = AegisVariationsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_AegisVariations[self.0]
    }
}

impl AegisVariationsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static AegisVariationsRow {
        &TABLE_AegisVariations[self.0]
    }
    pub fn get(&self) -> &'static AegisVariationsRow {
        &TABLE_AegisVariations[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_AegisVariations.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static AegisVariationsRow)> {
        TABLE_AegisVariations.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_AegisVariations.iter() {
            black_box(row);
        }
    }
}
