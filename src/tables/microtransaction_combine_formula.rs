#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MicrotransactionCombineFormula: LazyLock<Vec<MicrotransactionCombineFormulaRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/microtransactioncombineformula.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| MicrotransactionCombineFormulaRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#result_base_item_types_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#ingredients_base_item_types_keys: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| BaseItemTypesRef::new(value as usize)).collect()
            },
            r#bk2_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(40..40 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown48: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(48..48 + 16).unwrap();
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
            r#unknown64: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(64..64 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown68: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(68).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MicrotransactionCombineFormulaRow {
    pub r#id: String,
    pub r#result_base_item_types_key: BaseItemTypesRef,
    pub r#ingredients_base_item_types_keys: Vec<BaseItemTypesRef>,
    pub r#bk2_file: String,
    pub r#unknown48: Vec<i32>,
    pub r#unknown64: i32,
    pub r#unknown68: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MicrotransactionCombineFormulaRef(pub usize);

impl Deref for MicrotransactionCombineFormulaRef {
    type Target = MicrotransactionCombineFormulaRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MicrotransactionCombineFormula[self.0]
    }
}

impl MicrotransactionCombineFormulaRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MicrotransactionCombineFormulaRow {
        &TABLE_MicrotransactionCombineFormula[self.0]
    }
    pub fn get(&self) -> &'static MicrotransactionCombineFormulaRow {
        &TABLE_MicrotransactionCombineFormula[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MicrotransactionCombineFormula.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MicrotransactionCombineFormulaRow)> {
        TABLE_MicrotransactionCombineFormula.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_MicrotransactionCombineFormula.iter() {
            black_box(row);
        }
    }
}
