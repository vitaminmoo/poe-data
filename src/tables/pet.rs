#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_Pet: LazyLock<Vec<PetRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/pet.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| PetRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#mtx_type_game_specific: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MtxTypeGameSpecificRef::new(value as usize)
            },
            r#hash16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 2).unwrap();
                let value = cell_bytes.get_i16_le();
                value
            },
            r#hash32: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(26..26 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#size: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(30..30 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown34: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(34).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown35: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(35).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown36: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(36..36 + 16).unwrap();
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
            r#unknown52: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(52..52 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown68: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(68..68 + 16).unwrap();
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
            r#unknown84: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(84..84 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown88: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(88).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown89: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(89..89 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown105: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(105).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown106: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(106).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown107: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(107).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown108: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(108..108 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown116: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(116..116 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct PetRow {
    pub r#id: String,
    pub r#mtx_type_game_specific: MtxTypeGameSpecificRef,
    pub r#hash16: i16,
    pub r#hash32: i32,
    pub r#size: i32,
    pub r#unknown34: bool,
    pub r#unknown35: bool,
    pub r#unknown36: Vec<i64>,
    pub r#unknown52: i64,
    pub r#unknown68: Vec<i64>,
    pub r#unknown84: i32,
    pub r#unknown88: bool,
    pub r#unknown89: i64,
    pub r#unknown105: bool,
    pub r#unknown106: bool,
    pub r#unknown107: bool,
    pub r#unknown108: String,
    pub r#unknown116: i64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct PetRef(pub usize);

impl Deref for PetRef {
    type Target = PetRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_Pet[self.0]
    }
}

impl PetRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static PetRow {
        &TABLE_Pet[self.0]
    }
    pub fn get(&self) -> &'static PetRow {
        &TABLE_Pet[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Pet.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static PetRow)> {
        TABLE_Pet.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_Pet.iter() {
            black_box(row);
        }
    }
}
